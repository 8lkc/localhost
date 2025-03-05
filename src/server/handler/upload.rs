use {
    super::Handler,
    crate::{
        message::{Method, Request, Resource, Response},
        server::Middleware,
        utils::{AppErr, HttpErr, HttpResult, TEMPLATES},
    },
    lazy_static::lazy_static,
    regex::Regex,
    std::{
        collections::HashMap,
        env,
        fs::{self, File},
        io::Write,
        path::Path,
    },
    tera::Context,
};

// Debug function to print full request details
fn debug_request(req: &Request) {
    println!("=== DEBUG REQUEST ===");
    println!("Full request details:");
    println!("Content-Length: {}", req.headers.get("Content-Length").unwrap_or(&"N/A".to_string()));
    println!("Actual body length: {}", req.body.len());
    println!("Body first 100 chars: {}", &req.body[..100.min(req.body.len())]);
    
    // Print first 500 chars of body if not empty
    if !req.body.is_empty() {
        println!("Body (first 500 chars): {}", &req.body[..500.min(req.body.len())]);
    }
    
    // Check raw headers for additional information
    for (key, value) in &req.headers {
        println!("Header {}: {}", key, value);
    }
    println!("=== END DEBUG REQUEST ===");
}

// Improved regex definitions
lazy_static! {
    static ref BOUNDARY_REGEX: Regex = Regex::new(r"boundary=(.+)$").unwrap();
    static ref CONTENT_DISPOSITION_REGEX: Regex =
        Regex::new(r#"Content-Disposition: form-data; name="([^"]+)"(; filename="([^"]+)")?"#).unwrap();
    static ref CONTENT_TYPE_REGEX: Regex =
        Regex::new(r"Content-Type: (.+)\r\n").unwrap();
}

pub struct Upload;

impl Handler for Upload {
    fn handle(req: &Request) -> HttpResult<Response> {
        // Debug: print full request details
        debug_request(req);

        // Pre-validate middleware checks
        Middleware::check(req)
            .logger()?
            .method(Method::POST)?;
        
        let Resource::Path(_) = &req.resource;

        // Improved multipart content type detection
        match req.headers.get("Content-Type") {
            Some(content_type) if content_type.starts_with("multipart/form-data") => {
                println!("Detected multipart/form-data upload");
                Self::process_multipart_upload(req)
            },
            _ => {
                println!("Invalid content type for upload");
                Err(HttpErr::from(400)) // Bad request if not multipart
            }
        }
    }
}

impl Upload {
    fn process_multipart_upload(req: &Request) -> HttpResult<Response> {
          // Debug print for content type and length
    println!("Content-Type: {:?}", req.headers.get("Content-Type"));
    println!("Content-Length: {:?}", req.headers.get("Content-Length"));
    println!("Body Length: {}", req.body.len());
    
    // Print a small part of the body for debugging if not empty
    if !req.body.is_empty() {
        println!("Body starts with: {}", &req.body[..std::cmp::min(100, req.body.len())]);
    } else {
        println!("ERROR: Upload body is empty!");
        return Err(HttpErr::from(400)); // Bad Request
    }

        // More verbose error handling and diagnostics
        let content_type = req.headers.get("Content-Type")
            .ok_or_else(|| {
                println!("No Content-Type header found");
                HttpErr::from(400)
            })?;

        // Detailed boundary extraction
        let boundary = match BOUNDARY_REGEX.captures(content_type) {
            Some(caps) => {
                let boundary_value = caps[1].to_string();
                println!("Extracted Boundary: {}", boundary_value);
                format!("--{}", boundary_value)
            },
            None => {
                println!("Could not extract boundary from: {}", content_type);
                return Err(HttpErr::from(400));
            }
        };

        // Validate body
        if req.body.is_empty() {
            println!("ERROR: Upload body is empty!");
            return Err(HttpErr::from(400));
        }

        // Configure upload directory
        let default_dir = format!(
            "{}/public/uploads",
            env!("CARGO_MANIFEST_DIR")
        );
        let upload_dir = env::var("UPLOAD_DIR").unwrap_or(default_dir);

        // Ensure upload directory exists
        if !Path::new(&upload_dir).exists() {
            fs::create_dir_all(&upload_dir)
                .map_err(|e| {
                    println!("Failed to create upload directory: {:?}", e);
                    AppErr::from(e)
                })?;
        }

        // Split the body by boundary
        let parts: Vec<&str> = req.body.split(&boundary).collect();
        println!("Total parts found: {}", parts.len());

        let mut upload_results = Vec::new();

        // Process each part
        for (index, part) in parts.iter().enumerate().skip(1) {
            println!("Processing Part {}: {}", index, part);

            if part.starts_with("--") || part.is_empty() {
                println!("Skipping boundary or empty part");
                continue;
            }

            match CONTENT_DISPOSITION_REGEX.captures(part) {
                Some(caps) => {
                    let field_name = caps[1].to_string();
                    println!("Field Name: {}", field_name);

                    if let Some(filename_match) = caps.get(3) {
                        let filename = filename_match.as_str().to_string();
                        println!("Filename detected: {}", filename);

                        // Extract content type
                        let content_type_str = if let Some(ct_caps) = CONTENT_TYPE_REGEX.captures(part) {
                            ct_caps[1].to_string()
                        } else {
                            "application/octet-stream".to_string()
                        };

                        // Find the beginning of the actual file content
                        if let Some(content_start) = part.find("\r\n\r\n") {
                            let content = &part[(content_start + 4)..];

                            // Remove trailing \r\n
                            let content = if content.ends_with("\r\n") {
                                &content[..(content.len() - 2)]
                            } else {
                                content
                            };

                            // Save the file
                            let file_path = format!("{}/{}", upload_dir, filename);
                            println!("Attempting to save file: {}", file_path);

                            match File::create(&file_path) {
                                Ok(mut file) => {
                                    match file.write_all(content.as_bytes()) {
                                        Ok(_) => {
                                            println!("File saved successfully");
                                            upload_results.push(format!(
                                                "Uploaded {} ({} bytes, type: {})",
                                                filename,
                                                content.len(),
                                                content_type_str
                                            ));
                                        }
                                        Err(e) => {
                                            println!("File write error: {:?}", e);
                                        }
                                    }
                                }
                                Err(e) => {
                                    println!("File creation error: {:?}", e);
                                }
                            }
                        }
                    } else {
                        // Regular form field handling
                        if let Some(content_start) = part.find("\r\n\r\n") {
                            let content = &part[(content_start + 4)..];
                            let content = if content.ends_with("\r\n") {
                                &content[..(content.len() - 2)]
                            } else {
                                content
                            };

                            upload_results.push(format!(
                                "Form field: {} = {}",
                                field_name, content
                            ));
                        }
                    }
                }
                None => {
                    println!("No content disposition found in part");
                }
            }
        }

        // Render success response
        let mut ctx = Context::new();
        ctx.insert("title", "Upload Complete");
        ctx.insert("results", &upload_results);

        let content = TEMPLATES
            .render("upload_success.html", &ctx)
            .unwrap_or_else(|_| {
                let results_html = upload_results.join("<br>");
                format!(
                    "<html><head><title>Upload Complete</title></head>
                    <body><h1>Upload Complete</h1><p>{}</p>
                    <a href='/'>Back to home</a></body></html>",
                    results_html
                )
            });

        let mut headers = HashMap::new();
        headers.insert(
            "Content-Type".to_string(),
            "text/html".to_string(),
        );

        Ok(Response::ok(Some(headers), Some(content)))
    }

    // Serve the upload form
    pub fn serve_form() -> HttpResult<Response> {
        let ctx = Context::new();

        let content = TEMPLATES
            .render("upload.html", &ctx)
            .unwrap_or_else(|_| {
                String::from(
                    "<html><head><title>File Upload</title></head>
                    <body>
                      <h1>Upload File</h1>
                      <form action='/upload' method='post' enctype='multipart/form-data'>
                        <input type='file' name='file' required><br>
                        <input type='submit' value='Upload'>
                      </form>
                    </body></html>"
                )
            });

        let mut headers = HashMap::new();
        headers.insert(
            "Content-Type".to_string(),
            "text/html".to_string(),
        );

        Ok(Response::ok(Some(headers), Some(content)))
    }
}