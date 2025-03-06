use {
    super::{
        Cgi,
        Handler,
        Interpreters,
    },
    crate::{
        message::{
            Method,
            Request,
            Resource,
            Response,
        },
        server::Middleware,
        utils::{
            process_cgi_output,
            AppErr,
            HttpErr,
            HttpResult,
            INTERPRETERS,
        },
    },
    libc::{
        _exit,
        close,
        dup2,
        execvp,
        fork,
        pipe,
        waitpid,
        WEXITSTATUS,
        WIFEXITED,
    },
    std::{
        env,
        ffi::CString,
        fs::File,
        io::{
            self, Read, Write
        },
        os::fd::FromRawFd,
        path::{
            Path,
            PathBuf,
        }, process::Command,
    },
};

impl Cgi {
    pub fn has_valid_config(&self) -> bool { self.interpreters.is_some() }

    pub fn interprete_python(path: &str) -> Result<String, AppErr> {
        println!("Chemin reçu: {}", path);
        
        // Construire le chemin complet
        let full_path = format!("{}/{}", env!("CARGO_MANIFEST_DIR"), path);
        
        println!("Chemin complet: {}", full_path);
        
        let script_path = Path::new(&full_path);
        
        // Vérifier si le fichier existe
        if !script_path.exists() {
            println!("Le fichier n'existe pas: {}", full_path);
            return Err(AppErr::NotFound(io::Error::new(io::ErrorKind::NotFound, 
                format!("Fichier non trouvé: {}", full_path))));
        }
        
        // Exécuter le script Python directement sans fork/exec
        let output = Command::new("python3")
            .arg(&full_path)
            .current_dir(script_path.parent().unwrap_or(Path::new("/")))
            .output()
            .map_err(|e| AppErr::NotFound(io::Error::new(io::ErrorKind::Other, 
                format!("Échec de l'exécution: {}", e))))?;
        
        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            println!("Erreur Python: {}", error_msg);
            return Err(AppErr::NotFound(io::Error::new(io::ErrorKind::Other, 
                format!("Le script a échoué: {}", error_msg))));
        }
        
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(stdout)
    }
}

impl Handler for Cgi {
    /// Extracts the extension from the path and checks if it's a valid CGI
    /// script. Then prepares the path for the CGI script, extracts the
    /// PATH_INFO and QUERY_STRING. Then creates pipes for stdin/stdout
    /// from which it forks processes.
    fn handle(req: &Request) -> HttpResult<Response> {
        Middleware::check(req).logger()?;

        let Resource::Path(path) = &req.resource;

        let ext = path
            .split('.')
            .next_back()
            .ok_or(AppErr::ExtNotFound)?;

        let interpreter = INTERPRETERS
            .get(ext)
            .ok_or(AppErr::NoCGI)?;

        let script = format!(
            "{}/public{}",
            env!("CARGO_MANIFEST_DIR"),
            path
        );
        if !Path::new(&script).exists() {
            return Err(HttpErr::from(AppErr::NoCGI));
        }
        dbg!("ok");

        let script_buf = PathBuf::from(&script);
        let script_dir = script_buf
            .parent()
            .unwrap_or_else(|| Path::new("/"));

        let (path_info, query_str) = match path.find('?') {
            Some(pos) => (&path[..pos], &path[pos + 1..]),
            None => (path.as_str(), ""),
        };

        let mut stdin_pipe = [0; 2];
        let mut stdout_pipe = [0; 2];

        unsafe {
            if pipe(stdin_pipe.as_mut_ptr()) < 0 || pipe(stdout_pipe.as_mut_ptr()) < 0 {
                return Err(HttpErr::from(AppErr::last_os_error()));
            }
        }

        let pid = unsafe { fork() };

        if pid < 0 {
            // Fork failed
            Err(HttpErr::from(AppErr::last_os_error()))
        }
        else if pid > 0 {
            // Parent process
            unsafe {
                // Close unused pipe ends
                close(stdin_pipe[0]);
                close(stdout_pipe[1]);

                // Send request body to child.
                match req.method {
                    Method::POST => {
                        let mut writer = File::from_raw_fd(stdin_pipe[1]);
                        writer.write_all(&req.body.as_bytes())?;
                        // Signal EOF
                        drop(writer);
                    }
                    Method::GET => {
                        close(stdin_pipe[1]);
                    }
                    _ => {
                        return Err(HttpErr::from(405));
                    }
                };

                // Read CGI ouput
                let mut reader = File::from_raw_fd(stdout_pipe[0]);
                let mut output = String::new();
                reader.read_to_string(&mut output)?;

                // Wait for child to avoid zombies
                let mut status = 0;
                waitpid(pid, &mut status, 0);

                if WIFEXITED(status) && WEXITSTATUS(status) != 0 {
                    return Err(HttpErr::from(format!(
                        "CGI process exited with status {}",
                        WEXITSTATUS(status)
                    )));
                }

                // Parse CGI response
                process_cgi_output(&output)
            }
        }
        else {
            // Child process
            unsafe {
                // Close unused pipe ends
                close(stdin_pipe[1]);
                close(stdout_pipe[0]);

                // Redirect stdin/stdout
                dup2(stdin_pipe[0], 0);
                dup2(stdout_pipe[1], 1);

                // Close original pipe fds after duplication
                close(stdin_pipe[0]);
                close(stdout_pipe[1]);

                // Prepare environment
                env::set_current_dir(script_dir).unwrap_or(());
                env::set_var("SCRIPT_FILENAME", &script);
                env::set_var("SCRIPT_NAME", path);
                env::set_var("PATH_INFO", path_info);
                env::set_var("QUERY_STRING", query_str);
                env::set_var("REQUEST_METHOD", req.method.to_string());

                if let Some(content_type) = req
                    .headers
                    .get("Content-Type")
                {
                    env::set_var("CONTENT_TYPE", content_type);
                }
                if let Some(content_length) = req
                    .headers
                    .get("Content-Length")
                {
                    env::set_var("CONTENT_LENGTH", content_length);
                }

                for (key, value) in &req.headers {
                    let env_name = format!(
                        "HTTP_{}",
                        key.to_uppercase()
                            .replace('-', "_")
                    );
                    env::set_var(&env_name, value);
                }

                // Execute CGI script
                let interpreter_c = CString::new(*interpreter).unwrap();
                let script_c = CString::new(script).unwrap();
                let args = vec![interpreter_c.clone(), script_c, CString::new("").unwrap()];

                execvp(
                    interpreter_c.as_ptr(),
                    args.iter()
                        .map(|arg| arg.as_ptr())
                        .collect::<Vec<_>>()
                        .as_ptr(),
                );

                // If exec failed, exit with error
                _exit(1);
            }
        }
        // unreachable!("This should never be reached after fork");
    }
}
