use super::{
    AppErr,
    HttpErr,
};

impl From<AppErr> for HttpErr {
    fn from(value: AppErr) -> Self {
        match value {
            AppErr::NoCGI | AppErr::ExtNotFound => Self::from(404),
            _ => Self::from(500),
        }
    }
}

impl From<String> for HttpErr {
    fn from(message: String) -> Self {
        let msg = message.to_lowercase();

        let status_code = match msg {
            _ if msg.contains("bad request") => 400,
            _ if msg.contains("unauthorized") => 401,
            _ if msg.contains("forbidden") => 403,
            _ if msg.contains("not found") => 404,
            _ if msg.contains("method not allowed") => 405,
            _ => 500,
        };

        Self {
            status_code,
            message,
        }
    }
}

impl From<u16> for HttpErr {
    fn from(status_code: u16) -> Self {
        let message = match status_code {
            400 => "Bad Request".to_string(),
            401 => "Unauthorized".to_string(),
            403 => "Forbidden".to_string(),
            404 => "Not Found".to_string(),
            405 => "Method Not Allowed".to_string(),
            _ => "Internal Server Error".to_string(),
        };

        Self {
            status_code,
            message,
        }
    }
}
