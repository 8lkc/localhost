```
src/
├── main.rs             # Entry point
├── server/
│   ├── mod.rs          # Server implementation  
│   ├── config.rs       # Config structures
│   ├── virtual_host.rs # Virtual host handling
│   └── epoll.rs        # Event loop
├── http/
│   ├── request.rs      # Request parsing
│   ├── status.rs       # Status of the request
|   ├── mod.rs           
│   ├── response.rs     # Response building
│   └── headers.rs      # HTTP headers
├── handlers/
│   ├── cgi.rs          # CGI execution
│   ├── static.rs       # Static files
│   ├── directory.rs    # Directory structure
│   └── errors.rs       # Error pages
└── utils/
    ├── timeout.rs      # Connection timeout
    └── chunked.rs      # Chunked encoding

```