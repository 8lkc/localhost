use rand::{
    distributions::Alphanumeric,
    Rng,
};

pub fn generate_session_id() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect()
}

pub fn get_session_id(cookie: &str) -> Option<String> {
    cookie
        .split(';')
        .find(|s| {
            s.trim()
                .starts_with("session_id=")
        })
        .map(|s| s.trim()["session_id=".len()..].to_string())
}
