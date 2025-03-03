use {
    crate::utils::globals::{
        CLEANUP_FILE,
        SESSION_FILE,
    },
    std::{
        fs::File,
        io::{
            BufRead,
            BufReader,
            Write,
        },
    },
};

pub fn init_files(timestamp: u64) -> Result<(), String> {
    if !std::path::Path::new(SESSION_FILE).exists() {
        File::create(SESSION_FILE).map_err(|e| e.to_string())?;
    }
    if !std::path::Path::new(CLEANUP_FILE).exists() {
        let mut file =
            File::create(CLEANUP_FILE).map_err(|e| e.to_string())?;
        writeln!(file, "{}", timestamp).map_err(|e| {
            format!(
                "Impossible d'écrire le timestamp initial: {}",
                e
            )
        })?;
    }
    Ok(())
}

pub fn read_sessions() -> Result<Vec<String>, String> {
    let file = File::open(SESSION_FILE).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);
    Ok(reader
        .lines()
        .filter_map(Result::ok)
        .collect())
}

pub fn write_sessions(sessions: &[String]) -> Result<(), String> {
    let mut file =
        File::create(SESSION_FILE).map_err(|e| e.to_string())?;
    for session in sessions {
        writeln!(file, "{}", session)
            .map_err(|_| "Impossible d'écrire la session")?;
    }
    Ok(())
}
