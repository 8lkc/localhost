use {
    super::{
        router::Route,
        Server,
    },
    crate::{
        debug,
        utils::AppResult,
    },
    std::{
        collections::HashMap,
        net::{
            SocketAddr,
            TcpListener,
        },
        str::FromStr,
    },
};

impl Server {
    pub fn host(&self) -> &str {
        debug!(self.host.as_ref());
        self.host.as_ref().unwrap()
    }

    pub fn ports(&self) -> &Vec<usize> {
        self.ports.as_ref().unwrap()
    }

    pub fn root(&self) -> &str {
        self.root.as_ref().unwrap()
    }

    pub fn error_pages(&self) -> &Vec<String> {
        self.error_pages
            .as_ref()
            .unwrap()
    }

    pub fn uploads_max_size(&self) -> u64 {
        self.uploads_max_size.unwrap()
    }

    /// Common Gateway Interface
    pub fn cgi(&self) -> &HashMap<String, String> {
        self.cgi.as_ref().unwrap()
    }

    pub fn listing(&self) -> bool {
        self.listing.unwrap()
    }

    pub fn routes(&self) -> &Vec<Route> {
        self.routes.as_ref().unwrap()
    }

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
