use crate::{
    message::{
        Headers,
        Response,
    },
    utils::HttpResult,
};

pub fn process_cgi_output(output: &str) -> HttpResult<Response> {
    // Split headers and body
    if let Some(blank_line_pos) = output.find("\r\n\r\n") {
        let (headers_str, body) = output.split_at(blank_line_pos + 4);

        // Parse headers
        let mut headers = Headers::new();
        let mut status_code = 200;
        let mut status_txt = "OK".to_string();

        for line in headers_str.lines() {
            if line.is_empty() || line == "\r" {
                continue;
            }

            if let Some(pos) = line.find(':') {
                let (key, value) = line.split_at(pos);
                let value = value[1..].trim();

                if key.eq_ignore_ascii_case("Status") {
                    if let Some(pos) = value.find(' ') {
                        if let Ok(code) = value[..pos].parse::<u16>() {
                            status_code = code;
                            status_txt = value[pos + 1..].to_string();
                        }
                    }
                }
                else {
                    headers.insert(key.to_string(), value.to_string());
                }
            }
        }

        // Create response
        let mut response = Response::ok(Some(headers), Some(body.to_string()));
        response.set_status_code(status_code);
        response.set_status_txt(status_txt);
        Ok(response)
    }
    else {
        // No headers, treat as plain body
        Ok(Response::ok(None, Some(output.to_string())))
    }
}
