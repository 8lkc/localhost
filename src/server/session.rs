use {
    super::SessionStore,
    crate::utils::{
        generate_session_id,
        get_current_timestamp,
    },
    std::{
        collections::HashMap,
        time::Duration,
    },
};

impl SessionStore {
    pub fn new(timeout_minutes: u64) -> Self {
        Self {
            sessions: HashMap::new(),
            timeout:  Duration::from_secs(timeout_minutes * 60),
        }
    }

    pub fn create_session(&mut self) -> String {
        self.clean();
        let session_id = generate_session_id();
        let timestamp = get_current_timestamp();
        self.sessions
            .insert(session_id.clone(), timestamp);
        session_id
    }

    pub fn validate_session(&mut self, session_id: &str) -> bool {
        self.clean();
        self.sessions
            .get(session_id)
            .map_or(false, |&timestamp| {
                get_current_timestamp() - timestamp < self.timeout.as_secs()
            })
    }

    pub fn clean(&mut self) {
        let current_time = get_current_timestamp();
        self.sessions
            .retain(|_, &mut timestamp| current_time - timestamp < self.timeout.as_secs());
    }
}
