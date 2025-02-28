use {
    super::{
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
    pub fn has_valid_config(&self) -> bool {
        self.host.is_some()
            && self.ports.is_some()
            && self.root.is_some()
            && self.listing.is_some()
            && self
                .uploads_max_size
                .is_some()
            && self.router.is_some()
            && self
                .router
                .as_ref()
                .unwrap()
                .has_validate_config()
    }

    pub fn host(&self) -> &str {
        self.host.as_ref().unwrap()
    }

    pub fn ports(&self) -> &Vec<usize> {
        self.ports.as_ref().unwrap()
    }

    pub fn root(&self) -> &str {
        self.root.as_ref().unwrap()
    }

    pub fn uploads_max_size(&self) -> u64 {
        self.uploads_max_size.unwrap()
    }

    pub fn listing(&self) -> bool {
        self.listing.unwrap()
    }

    pub fn router(&self) -> &Router {
        self.router.as_ref().unwrap()
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
