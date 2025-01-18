use {
    std::collections::HashMap,
    localhost::{
        Method,
        Request,
        Resource,
    },
};

#[test]
fn test_method_into() {
    let m: Method = "GET".into();
    assert_eq!(m, Method::GET);
}

#[test]
fn test_read_http() {
    let s = String::from(
        "GET /greeting HTTP/1.1\r\nHost: localhost:3000\r\nUser-Agent: curl/7.64.1\r\nAccept: \
         */*\r\n\r\n",
    );

    let mut headers_expected = HashMap::new();
    headers_expected.insert("Host".into(), "localhost:3000".into());
    headers_expected.insert("User-Agent".into(), "curl/7.64.1".into());
    headers_expected.insert("Accept".into(), "*/*".into());

    let req: Request = s.into();
    assert_eq!(Method::GET, req.method);
    assert_eq!(
        Resource::Path("/greeting".to_string()),
        req.resource
    );
    assert_eq!(headers_expected, req.headers);
}
