use std::collections::HashMap;
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::os::unix::io::{AsRawFd, RawFd};
use libc::{epoll_create1, epoll_ctl, epoll_wait, EPOLLIN, EPOLL_CTL_ADD, EPOLL_CTL_DEL};

const MAX_EVENTS: usize = 32;
const MAX_READ_BUFFER: usize = 4096;

struct EpollManager {
    epoll_fd: RawFd,
    clients: HashMap<RawFd, TcpStream>,
}

impl EpollManager {
    fn new() -> io::Result<Self> {
        let epoll_fd = unsafe { epoll_create1(0) };

        if epoll_fd < 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(EpollManager {
            epoll_fd,
            clients: HashMap::new(),
        })
    }

    fn add_server(&mut self, listener: &TcpListener) -> io::Result<()> {
        let fd = listener.as_raw_fd();
        let mut event: libc::epoll_event = unsafe { std::mem::zeroed() };
        event.events = EPOLLIN as u32;
        event.u64 = fd as u64;

        if unsafe { epoll_ctl(self.epoll_fd, EPOLL_CTL_ADD, fd, &mut event) } < 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }

    fn run(&mut self, mut servers: Vec<Server>) -> io::Result<()> {
        let mut events: Vec<libc::epoll_event> = Vec::with_capacity(MAX_EVENTS);
        unsafe { events.set_len(MAX_EVENTS) };

        // Ajouter tous les serveurs à epoll
        for server in &servers {
            server.listener.set_nonblocking(true)?;
            self.add_server(&server.listener)?;
        }

        println!("Tous les serveurs sont démarrés et en attente de connexions...");

        loop {
            let nfds = unsafe { epoll_wait(self.epoll_fd, events.as_mut_ptr(), MAX_EVENTS as i32, -1) };
            if nfds < 0 {
                return Err(io::Error::last_os_error());
            }

            for n in 0..nfds as usize {
                let fd = events[n].u64 as RawFd;
                
                // Vérifier si c'est un serveur qui a un événement
                for server in &mut servers {
                    if server.listener.as_raw_fd() == fd {
                        // Nouvelle connexion
                        match server.listener.accept() {
                            Ok((mut stream, addr)) => {
                                println!("Nouvelle connexion depuis {} sur {}", addr, server.name);
                                stream.set_nonblocking(true)?;
                                let stream_fd = stream.as_raw_fd();
                                
                                let mut event: libc::epoll_event = unsafe { std::mem::zeroed() };
                                event.events = EPOLLIN as u32;
                                event.u64 = stream_fd as u64;

                                if unsafe { epoll_ctl(self.epoll_fd, EPOLL_CTL_ADD, stream_fd, &mut event) } < 0 {
                                    return Err(io::Error::last_os_error());
                                }

                                self.clients.insert(stream_fd, stream);
                            }
                            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {}
                            Err(e) => eprintln!("Erreur d'acceptation: {}", e),
                        }
                        break;
                    }
                }

                // Gérer les clients existants
                if let Some(stream) = self.clients.get_mut(&fd) {
                    let mut buffer = [0; MAX_READ_BUFFER];
                    match stream.read(&mut buffer) {
                        Ok(0) | Err(_) => {
                            // Connexion fermée ou erreur
                            unsafe { epoll_ctl(self.epoll_fd, EPOLL_CTL_DEL, fd, std::ptr::null_mut()) };
                            self.clients.remove(&fd);
                        }
                        Ok(n) => {
                            // Echo simple des données
                            if let Err(e) = stream.write_all(&buffer[..n]) {
                                eprintln!("Erreur d'écriture: {}", e);
                                unsafe { epoll_ctl(self.epoll_fd, EPOLL_CTL_DEL, fd, std::ptr::null_mut()) };
                                self.clients.remove(&fd);
                            }
                        }
                    }
                }
            }
        }
    }
}

// Adapter votre structure Server existante
struct Server {
    name: String,
    listener: TcpListener,
}

impl Server {
    fn new(name: String, addr: &str) -> io::Result<Self> {
        let listener = TcpListener::bind(addr)?;
        Ok(Server { name, listener })
    }
}

fn main() -> io::Result<()> {
    let mut epoll_manager = EpollManager::new()?;
    let mut servers = Vec::new();

    // Convertir vos serveurs existants
    for server in mux.servers {
        let addr = format!("{}:{}", server.host, server.port); // Adaptez selon votre configuration
        match Server::new(server.name.clone(), &addr) {
            Ok(server) => {
                println!("Serveur {} configuré sur {}", server.name, addr);
                servers.push(server);
            }
            Err(e) => eprintln!("Erreur lors de la création du serveur {}: {}", server.name, e),
        }
    }

    // Lancer tous les serveurs avec epoll
    epoll_manager.run(servers)
}