<h1 align="center">
    <img src="assets/computing.png" alt="computing">
    <br>LOCALHOST<br>
    <img src="assets/ferris.svg" alt="Rust-logo">
</h1>

Here, the aim is to `understand how internet works from the server side` by learning the basics of the Hypertext Transfer Protocol (`HTTP`).

## 🗃️ Table of Contents

1.  [Overview](#overview)
    - [Features](#features)
2.  [Project Structure](#project-structure)
3.  [Usage](#usage)
    - [Installation](#installation)
    - [Launch](#launch)
4.  [License](#license)

## 1️⃣ Overview

This is a `high-performance HTTP server written in Rust`. Designed for **scalability** and **robustness**, it supports multiple concurrent connections using non-blocking I/O and epoll, handles static file serving, dynamic CGI execution, and advanced HTTP features (such as cookie/session management and file uploads). The server aims to be a solid foundation for both learning and production-grade projects.

### ➖ Features

Localhost is built in Rust to leverage its safety and concurrency features. This project is designed to:

- **Handle multiple client connections:** Using `non-blocking sockets` and `epoll` for efficient I/O multiplexing.
- **Support HTTP/1.1:** Parsing and generating well-formed HTTP `requests` and `responses`.
    - **HTTP Request Parsing:** Extracting HTTP methods, URLs, headers, and bodies (including support for chunked requests).
    - **HTTP Response Handling:** Generating responses with correct status codes and headers (e.g., `200 OK`, `404 Not Found`, `500 Internal Server Error`).
- **Serve static files:** Deliver HTML, CSS, JavaScript, and image files. Directory routing, MIME type handling, and default file management.
- **Execute CGI scripts:** Run dynamic content with support for Python, PHP, and more. Execute and capture output from dynamic scripts.
- **File Uploads:** Manage POST requests with file uploads and enforce payload size limits.
- **Cookie and Session Management:** Unique cookies per visitor and session tracking.
- **Dynamic Configuration:** Read from configuration files (e.g., `config.toml`) for server settings.
- **Robust Testing:** Includes stress tests (using tools like siege) and error handling to ensure high availability.

The project will be designed with stress testing and memory safety in mind. This is to make it **robust and scalable**.

## 2️⃣ Project Structure

```bash
    .
    ├──── assets
    │     ├──── computing.png
    │     ├──── ferris.svg
    │     └──── header.txt
    ├──── roadmap
    │     ├──── audit.todo
    │     └──── tasks.todo
    ├──── src
    │     ├──── cgi-bin
    │     ├──── handlers
    │     │     ├──── errors.rs
    │     │     ├──── mod.rs
    │     │     └──── server.rs
    │     ├──── http
    │     │     └──── mod.rs
    │     ├──── server
    │     │     ├──── config.rs
    │     │     ├──── config.toml   # Configuration file
    │     │     ├──── epoll.rs
    │     │     └──── mod.rs
    │     ├──── static
    │     └──── main.rs             # Entry point
    ├──── .gitignore
    ├──── Cargo.lock
    ├──── Cargo.toml
    ├──── makefile
    └──── README.md
```

## 3️⃣ Usage

### ➖ Installation

First of all make sure to have access to the `project repository`. **Then apply the following process:**

1.  **Clone the repository:**
```bash
git clone https://learn.zone01dakar.sn/git/npouille/localhost.git
```

2.  **Move to the project directory:**
```bash
cd localhost
```

### ➖ Launch

- **Start the server:**
```bash
make
```
This command will execute the commands contained in the `makefile`.
