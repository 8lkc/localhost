use std::{collections::HashMap, io::{Read, Write}};

use libc::{epoll_create1, epoll_ctl, epoll_event, epoll_wait, EPOLL_CTL_ADD};
use mio::net::{TcpListener, TcpStream};
use server::config::Servers;
use std::os::fd::AsRawFd;

mod server;

// use server::{
//     Server, 
//     config::*
// };

fn main() /* -> std::io::Result<()> */ {
    //* Step 1 : Create `epoll`
    //*---------------------------------------------
    let epoll_descriptor = unsafe {epoll_create1(0)};
    if epoll_descriptor < 0 {
        panic!("Error creating epoll descriptor");
    }

    // Configuration de base
    let configs = Servers::new();
    // Ensure there is at least one configuration.
    if configs.servers.is_empty() {panic!("No server configuration found")}

    //* Step 2 : Creating multiple TCP servers
    //*---------------------------------------------
    let addresses = configs.servers.iter().map(|config| {
        config.ports.iter().map(|port| {format!("{}:{}", config.host, port)}).collect::<Vec<String>>()
    }).flatten().collect::<Vec<String>>();
    let mut servers = HashMap::new();

    for address in &addresses {
        let std_listener = std::net::TcpListener::bind(address).expect("Unable to start server");
        std_listener.set_nonblocking(true).expect("Cannot set non-blocking");
        let listener = TcpListener::from_std(std_listener);
    
        let fd = listener.as_raw_fd();
        let mut event = libc::epoll_event {
            events: libc::EPOLLIN as u32, // Monitoring new connections
            u64: fd as u64 // Store the FD in epoll
        };

        unsafe {
            if epoll_ctl(epoll_descriptor, EPOLL_CTL_ADD, fd, &mut event) < 0 {
                panic!("Error adding file descriptor to epoll");
            }
        }
        servers.insert(fd, listener);
    }

    println!("Servers listening on {:#?}", addresses);

    //* Step 3 : Main loop `epoll_wait`
    //*---------------------------------------------
    let mut events: [epoll_event; 1024] = unsafe {std::mem::zeroed()};

    loop {
        let num_events = unsafe {epoll_wait(epoll_descriptor, events.as_mut_ptr(), events.len() as i32, -1)};
        if num_events < 0 {panic!("ERROR: epoll_wait");}

        for i in 0..num_events as usize {
            let fd = events[i].u64 as i32;

            if let Some(listener) = servers.get(&fd) {
                if let Ok((stream, address)) = listener.accept() {
                    println!("New connection from: {:?}", address);
                    handle_client(stream);
                }
            }
        }
    }

    // // Configuration de base
    // let configs = Servers::new();
    // // Ensure there is at least one configuration.
    // if configs.servers.is_empty() {panic!("No server configuration found")}

    // // Loop over each server configuration and start the servers.
    // for config in configs.servers {
    //     for port in config.ports.iter() {
    //         println!("Server named {} starting on {}:{}", config.name, config.host, port);
    //     }
    //     let mut server = Server::new(config)?;
    //     server.start()?;
    // }

    // Ok(())
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let response = "HTTP/1.1 200 OK\r\n\r\nHello, world!";
    stream.write_all(response.as_bytes()).unwrap();
}
