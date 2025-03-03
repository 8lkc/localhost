use {
    super::SessionStore,
    crate::utils::{
        generate_session_id,
        get_current_timestamp,
        get_last_cleanup,
        init_files,
        read_sessions,
        update_last_cleanup,
        write_sessions,
    },
    lazy_static::lazy_static,
    std::time::Duration,
};

impl SessionStore {
    pub fn new(timeout_minutes: u64) -> Result<Self, String> {
        let current_time = get_current_timestamp();
        init_files(current_time)?;
        Ok(Self {
            timeout:          Duration::from_secs(timeout_minutes * 60),
            cleanup_interval: 120,
        })
    }

    pub fn create_session(&self) -> Result<String, String> {
        if let Err(e) = self.clean() {
            dbg!("Erreur lors du nettoyage des sessions :", e);
        }

        let session_id = generate_session_id();
        let timestamp = get_current_timestamp();

        let mut sessions = read_sessions()?;
        sessions.push(format!("{}:{}", session_id, timestamp));

        write_sessions(&sessions)?;

        Ok(session_id)
    }

    pub fn validate_session(&self, session_id: &str) -> bool {
        if let Err(e) = self.clean() {
            dbg!("Erreur lors du nettoyage de la session", e);
        }

        if let Ok(sessions) = read_sessions() {
            let current_time = get_current_timestamp();

            for line in sessions {
                let parts: Vec<&str> = line.split(':').collect();
                if parts.len() == 2 {
                    let (id, timestamp_str) = (parts[0], parts[1]);
                    if id == session_id {
                        if let Ok(timestamp) = timestamp_str.parse::<u64>()
                        {
                            return current_time - timestamp
                                < self.timeout.as_secs();
                        }
                    }
                }
            }
        }
        false
    }

    pub fn clean(&self) -> Result<(), String> {
        let current_time = get_current_timestamp();
        let last_cleanup = get_last_cleanup();

        if current_time - last_cleanup >= self.cleanup_interval {
            update_last_cleanup(current_time)?;
            if let Ok(sessions) = read_sessions() {
                let valid_sessions: Vec<String> = sessions
                    .into_iter()
                    .filter(|line| {
                        let parts: Vec<&str> = line.split(':').collect();
                        if parts.len() == 2 {
                            if let Ok(timestamp) = parts[1].parse::<u64>()
                            {
                                return current_time - timestamp
                                    < self.timeout.as_secs();
                            }
                        }
                        false
                    })
                    .collect();

                write_sessions(&valid_sessions)?;
            }
        }
        Ok(())
    }
}

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
                timeout:          Duration::from_secs(60),
                cleanup_interval: 120,
            }
        }
    };
}
