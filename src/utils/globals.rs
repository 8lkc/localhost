use std::{
    collections::HashMap,
    sync::LazyLock,
};

pub const TIMEOUT: u64 = 1000;
pub static INTERPRETERS: LazyLock<HashMap<&str, &str>> =
    LazyLock::new(|| HashMap::from([("py", "python3")]));
