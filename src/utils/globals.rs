use {
    lazy_static::lazy_static,
    std::{
        collections::HashMap,
        sync::LazyLock, time::Duration,
    },
    tera::Tera,
};
use crate::server::session::SessionStore; 

pub const TIMEOUT: u64 = 1000;

pub static INTERPRETERS: LazyLock<HashMap<&str, &str>> =
    LazyLock::new(|| HashMap::from([("py", "python3")]));

pub static TEMPLATES: LazyLock<Tera> = LazyLock::new(|| {
    let root = env!("CARGO_MANIFEST_DIR");
    let full_path = format!("{}/public/templates/*.html", root);
    let mut tera = match Tera::new(&full_path) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Failed to load templates: {}", e);
            std::process::exit(1);
        }
    };
    tera.autoescape_on(vec!["html"]);
    tera
});

lazy_static! {
    pub static ref SESSION_STORE: SessionStore = match SessionStore::new(1)
    {
        Ok(store) => store,
        Err(err) => {
            dbg!(
                "Impossible de créer le store de session",
                err
            );
            SessionStore {
                timeout: Duration::from_secs(60),
                cleanup_interval: 120,
            }
        }
    };
}