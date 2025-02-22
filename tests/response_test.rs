use {
    localhost::Response,
    std::collections::HashMap,
};

#[test]
fn test_response_struct_creation_200() {
    let response_actual = Response::new(
        200,
        None,
        Some("Item was shipped on 21st Dec 2020".into()),
    );

    assert_eq!(response_actual.status_code(), 200);
    assert_eq!(response_actual.status_text(), "OK");
    assert_eq!(
        response_actual.headers(),
        "Content-Type:text/html\r\n"
    );
    assert_eq!(
        response_actual.body(),
        "Item was shipped on 21st Dec 2020"
    );
}

#[test]
fn test_response_struct_creation_404() {
    let response_actual = Response::new(
        404,
        None,
        Some("Item was shipped on 21st Dec 2020".into()),
    );

    assert_eq!(response_actual.status_code(), 404);
    assert_eq!(response_actual.status_text(), "Not Found");
    assert_eq!(
        response_actual.headers(),
        "Content-Type:text/html\r\n"
    );
    assert_eq!(
        response_actual.body(),
        "Item was shipped on 21st Dec 2020"
    );
}

#[test]
fn test_http_response_creation() {
    let mut h = HashMap::new();
    h.insert("Content-Type", "text/html");

    let response_expected = Response::new(
        404,
        Some(h),
        Some("Item was shipped on 21st Dec 2020".into()),
    );

    let http_string: String = response_expected.into();
    let response_actual = "HTTP/1.1 404 Not \
                           Found\r\nContent-Type:text/html\r\\
                           nContent-Length: 33\r\n\r\nItem was shipped on \
                           21st Dec 2020";

    assert_eq!(http_string, response_actual);
}
