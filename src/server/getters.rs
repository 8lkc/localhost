use {
    super::{
        Route,
        Router,
        Server,
    },
    crate::utils::AppResult,
    std::{
        net::{
            SocketAddr,
            TcpListener,
        },
        str::FromStr,
    },
};

impl Server {
    pub fn host(&self) -> &str { self.host.as_ref().unwrap() }

    pub fn ports(&self) -> &Vec<usize> { self.ports.as_ref().unwrap() }

    pub fn root(&self) -> &str { self.root.as_ref().unwrap() }

    pub fn upload_max_size(&self) -> u64 { self.uploads.unwrap() }

    pub fn listing(&self) -> bool { self.listing.unwrap() }

    pub fn router(&self) -> &Router { self.router.as_ref().unwrap() }

    pub fn listeners(&self) -> AppResult<Vec<TcpListener>> {
        let mut listeners = vec![];
        let host = self.host();

        for port in self.ports() {
            let address = SocketAddr::from_str(&format!("{host}:{port}"))?;
            listeners.push(TcpListener::bind(address)?);
        }

        Ok(listeners)
    }
}

impl Router {
    pub fn routes(&self) -> &Vec<Route> { self.routes.as_ref().unwrap() }

    pub fn get_session(&self, path: &str) -> bool {
        for route in self.routes() {
            if let Some(path_route) = &route.path {
                if path_route == path {
                    return route.check_session();
                }
            }
        }
        false
    }
}

impl Route {
    pub fn path(&self) -> &str { self.path.as_ref().unwrap() }

    pub fn allowed_methods(&self) -> &Vec<String> {
        self.methods.as_ref().unwrap()
    }

    pub fn check_session(&self) -> bool { self.session.unwrap() }
}
