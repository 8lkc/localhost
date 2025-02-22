use {
    super::{
        AppErr,
        HttpErr,
    },
    std::io,
};

impl From<io::Error> for HttpErr {
    fn from(_error: io::Error) -> Self { Self::from(500) }
}

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
        let msg = match status_code {
            400 => "Bad Request",
            401 => "Unauthorized",
            403 => "Forbidden",
            404 => "Not Found",
            405 => "Method Not Allowed",
            _ => "Internal Server Error",
        };

        Self {
            status_code,
            message: msg.to_string(),
        }
    }
}
