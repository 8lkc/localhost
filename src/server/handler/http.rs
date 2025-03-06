use {
    super::{FileSystem, Handler, Http},
    crate::{
        debug,
        message::{Headers, Method, Request, Resource, Response},
        server::{handler::Cgi, Middleware, SessionStore},
        utils::{get_session_id, AppErr, HttpErr, HttpResult, HTTP, TEMPLATES},
    },
    std::{fmt::format, fs, path::Path},
    tera::Context,
};

impl Handler for Http {
    fn handle(req: &Request) -> HttpResult<Response> {
        Middleware::check(req)
            .logger()?
            .method(Method::GET)?;

        // Get the path of static page resource being requested
        let Resource::Path(s) = &req.resource;

        // Parse the URI
        let route: Vec<&str> = s.split("/").collect();
        let mut root_directory = format!("public{}", s);
        if route.len() > 1 && route[1] == "public" {
            root_directory = format!("public/{}", route[2..].join("/"));
        }
        match !s.contains(".") {
            true => Self::serve_default("index.html", &root_directory),
            _ => Self::serve_static(&root_directory),
        }
    }
}

impl Http {
    pub fn new(timeout_minutes: u64) -> Self {
        Self {
            session_store: SessionStore::new(timeout_minutes),
        }
    }

    fn serve_default(tmpl: &str, route: &str) -> HttpResult<Response> {
        let mut ctx = Context::new();
        ctx.insert("title", "Rust");

        let items = FileSystem::listing(route)?;
        ctx.insert("list", &items);

        let page = TEMPLATES
            .render(&tmpl, &ctx)
            .map_err(|e| AppErr::from(debug!(e)))?;

        Ok(Response::ok(None, Some(page)))
    }

    fn serve_static(path: &str) -> HttpResult<Response> {
        let mut headers = Headers::new();
        let filepath: Vec<&str> = path.split("/").collect();
        let final_path: String = filepath[1..].join("/");
        let mut cgi = false;
        // Déterminer le Content-Type en fonction de l'extension du fichier
        let content_type = match final_path.split('.').last() {
            Some("css") => "text/css",
            Some("js") => "text/javascript",
            Some("html") => "text/html",
            Some("json") => "application/json",
            Some("png") => "image/png",
            Some("jpg") | Some("jpeg") => "image/jpeg",
            Some("gif") => "image/gif",
            Some("py") => {
                cgi = true;
                "text/plain"
            }
            Some("svg") => "image/svg+xml",
            Some("txt") => "text/plain",
            Some("pdf") => "application/pdf",
            Some("xml") => "application/xml",
            Some("zip") => "application/zip",
            _ => "text/plain", // Type par défaut pour les fichiers inconnus
        };

        // Ajouter le Content-Type aux en-têtes
        headers.insert(
            "Content-Type".to_string(),
            content_type.to_string(),
        );

        println!("final path{}", final_path);

        if !Path::exists(Path::new(&final_path)) {
            let _ = HttpErr::from(404);
        }
        // Charger le contenu du fichier
        let content = fs::read_to_string(&final_path)?;

        // Retourner la réponse avec les en-têtes et le contenu
        if cgi {
            match Cgi::interprete_python(&final_path) {
                Ok(res) => {
                    // Traitement en cas de succès avec la sortie du script Python
                    Ok(Response::ok(Some(headers), Some(res)))
                    // Utilisez res comme nécessaire
                }
                Err(err) => {
                    // Journaliser l'erreur et retourner une réponse d'erreur
                    eprintln!("Erreur d'exécution Python: {:?}", err);
                    return Err(debug!(HttpErr::from(500)));
                }
            }
        } else {
            Ok(Response::ok(Some(headers), Some(content)))
        }
    }
    
    pub fn has_valid_session(&mut self, req: &Request) -> bool {
        if let Some(cookie) = req.headers.get("Cookie") {
            if let Some(session_id) = get_session_id(cookie) {
                return self
                    .session_store
                    .validate_session(&session_id.clone());
            }
        }
        false
    }

    pub fn serve_auth(path: &str) -> HttpResult<Response> {
        let session_id = match HTTP.write() {
            Ok(mut http) => http
                .session_store
                .create_session(),
            Err(e) => {
                dbg!("Error writing response: {}", e);
                return Err(debug!(HttpErr::from(500)));
            }
        };
        let mut headers = Headers::new();

        headers.insert(
            "Set-Cookie".to_string(),
            format!("session_id={}; Path=/; HttpOnly", session_id),
        );

        headers.insert(
            "Content-Type".to_string(),
            "text/html".to_string(),
        );

        let mut ctx = Context::new();
        ctx.insert("title", "Authentication");

        let page = TEMPLATES
            .render(&path, &ctx)
            .map_err(|e| AppErr::from(debug!(e)))?;

        Ok(Response::ok(Some(headers), Some(page)))
    }
}
