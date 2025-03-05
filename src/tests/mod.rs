#[cfg(test)]
mod tests {
    use crate::{Method, Request, Resource, Response};

    use super::*;
    use std::{collections::HashMap, io::Cursor};

    #[test]
    fn test_chunked_request_parsing() {
        let chunked_request = 
            "POST /upload HTTP/1.1\r\n\
             Host: example.com\r\n\
             Transfer-Encoding: chunked\r\n\
             \r\n\
             4\r\n\
             Wiki\r\n\
             5\r\n\
             pedia\r\n\
             0\r\n\
             \r\n";

        let mut cursor = Cursor::new(chunked_request);
        let request = Request::parse_chunked(cursor).unwrap();
        
        assert_eq!(request.method, Method::POST);
        assert_eq!(request.resource, Resource::Path("/upload".to_string()));
        assert_eq!(request.msg_body, "Wikipedia");
    }

    #[test]
    fn test_response_chunking() {
        let mut output = Vec::new();
        let response = Response {
            status_code: 200,
            status_text: "OK".to_string(),
            headers: Some(HashMap::from([
                ("Content-Type".to_string(), "text/plain".to_string())
            ])),
            body: Some("Hello, World!".to_string()),
        };

        response.send_chunked(&mut output, 5).unwrap();
        
        let response_str = String::from_utf8(output).unwrap();
        assert!(response_str.contains("Transfer-Encoding: chunked"));
        assert!(response_str.contains("5\r\nHello\r\n"));
    }
}