use {
    super::{
        Session,
        SessionStore,
    },
    crate::utils::generate_session_id,
    std::{
        collections::HashMap,
        sync::{
            Arc,
            Mutex,
        },
        time::{
            Duration,
            Instant,
        },
    },
};

impl SessionStore {
    pub fn new(timeout_minutes: u64) -> Self {
        Self {
            sessions: Arc::new(Mutex::new(HashMap::new())),
            timeout:  Duration::from_secs(timeout_minutes * 60),
        }
    }

    pub fn create_session(&self) -> String {
        let session_id = generate_session_id();
        let session = Session {
            created_at: Instant::now(),
        };

        let mut sessions = self.sessions.lock().unwrap();
        sessions.insert(session_id.clone(), session);

        session_id
    }

    pub fn validate_session(&self, session_id: &str) -> bool {
        let sessions = self.sessions.lock().unwrap();
        if let Some(session) = sessions.get(session_id) {
            session.created_at.elapsed() < self.timeout
        }
        else {
            false
        }
    }

    pub fn cleanup_expired_sessions(&self) {
        let mut sessions = self.sessions.lock().unwrap();
        sessions.retain(|_, session| {
            session.created_at.elapsed() < self.timeout
        });
    }
}
