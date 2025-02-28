use {
    super::{
        AppErr,
        HttpErr,
    },
    std::io,
};

impl From<io::Error> for HttpErr {
    fn from(_error: io::Error) -> Self {
        Self::from(500)
    }
}

impl From<AppErr> for HttpErr {
    fn from(value: AppErr) -> Self {
        match value {
            AppErr::NoCGI
            | AppErr::ExtNotFound
            | AppErr::NotFound(_)
            | AppErr::TmplNotFound(_) => Self::from(404),
            AppErr::Custom(msg) => Self::from(msg),
            _ => Self::from(500),
        }
    }
}

impl From<String> for HttpErr {
    fn from(message: String) -> Self {
        let msg = message.to_lowercase();

        let status_code = match msg {
            _ if msg.contains("bad request") => 400,
            _ if msg.contains("forbidden") => 403,
            _ if msg.contains("not found") => 404,
            _ if msg.contains("method not allowed") => 405,
            _ if msg.contains("too large") => 413,
            _ => 500,
        };

        Self {
            status_code,
            message,
        }
    }
}

impl From<u32> for HttpErr {
    fn from(status_code: u32) -> Self {
        let msg = match status_code {
            400 => "Bad Request",
            403 => "Forbidden",
            404 => "Not Found",
            405 => "Method Not Allowed",
            413 => "Request Entity Too Large",
            _ => "Internal Server Error",
        };

        Self {
            status_code,
            message: msg.to_string(),
        }
    }
}
