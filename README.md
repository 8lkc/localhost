<h1 align=center>
    localhost
    <br>
    <img alt="Ferris" src="assets/ferris.svg">
</h1>

## Table of Contents

- [Table of Contents](#table-of-contents)
- [Tech Stack](#tech-stack)
- [Overview](#overview)
- [Installation](#installation)
  - [Cloning](#cloning)
  - [File System](#file-system)
- [Usage](#usage)
  - [Classes](#classes)
  - [Sequence](#sequence)
- [Contributors](#contributors)
  - [Authors](#authors)
  - [Peers](#peers)
  - [Testers](#testers)
  - [Auditors](#auditors)
- [License](#license)

## Tech Stack

[![RUST](https://img.shields.io/badge/Rust-black?style=for-the-badge&logo=rust&logoColor=#E57324)](./src/main.rs)
[![SHELL SCRIPT](https://img.shields.io/badge/Shell_Script-121011?style=for-the-badge&logo=gnu-bash&logoColor=white)](./scripts/gitify.sh)
[![MARKDOWN](https://img.shields.io/badge/Markdown-000000?style=for-the-badge&logo=markdown&logoColor=white)](#table-of-contents)

## Overview

## Installation

### Cloning

```shell
git clone http://learn.zone01dakar.sn/git/fakeita/localhost
Cloning into 'localhost'...
warning: redirecting to https://learn.zone01dakar.sn/git/fakeita/localhost/
remote: Enumerating objects: 15, done.
remote: Counting objects: 100% (15/15), done.
remote: Compressing objects: 100% (11/11), done.
remote: Total 15 (delta 0), reused 0 (delta 0), pack-reused 0
Receiving objects: 100% (15/15), done.

cd localhost
tree --dirsfirst

```

### File System

    --📂./
        |
        +-📂 assets/
        |       |
        |       +-🌄 ferris.svg
        |
        +-📂 config/
        |       |
        |       +-⚙️ server.toml
        |
        +-📂 pages/
        |       |
        |       +-📄 error.html
        |       +-📄 index.html
        |
        +-📂 scripts/
        |       |
        |       +-📜 gitify.sh
        |       +-📜 utils.sh
        |
        +---📂 src/
        |       |
        |       +---📂 cgi/
        |       |       |
        |       |       +-📄 handlers.rs
        |       |       +-📄 mod.rs
        |       |
        |       +--📂 http/
        |       |       |
        |       |       +-📄 mod.rs
        |       |       +-📄 request.rs
        |       |       +-📄 response.rs
        |       |       +-📄 status.rs
        |       |
        |       +-📂 server/
        |       |       |
        |       |       +-📄 config.rs
        |       |       +-📄 connection.rs
        |       |       +-📄 epoll.rs
        |       |       +-📄 handler.rs
        |       |       +-📄 mod.rs
        |       |
        |       +-📂 utils/
        |       |       |
        |       |       +-📄 error.rs
        |       |       +-📄 logging.rs
        |       |
        |       +-📄 lib.rs
        |       +-📄 main.rs
        |
        +-📂 todos/
        |       |
        |       +-📝 audit.todo
        |       +-📝 instructions.todo
        |       +-📝 rules.todo
        |       +-📝 tasks.todo
        |
        +-🚫 .gitignore
        +-🔒 Cargo.lock
        +-⚙️ Cargo.toml
        +-🔑 LICENSE
        +-📖 README.md
        +-⚙️ rustfmt.toml

## Usage

### Classes

```mermaid
classDiagram
direction LR

class Handler {
  <<trait>>
  +handle(request) Response
  +load_file(file_name) String
}

class From {
  <<trait>>
  +from(str) Self
  +from(Response) String
}

class Default {
  <<trait>>
  +default() Self
}

class Server {
  <<struct>>
  socket_adrr
  +new(socket_addr) Server
  +run()
}

class Router {
  <<struct>>
  +route(req, stream) 
}

class WebService {
  <<struct>>
  +load_json() [Data]
}

class StaticPage
<<struct>> StaticPage

class ErrorPage
<<struct>> ErrorPage

class Request {
  <<struct>>
  +method
  +resource
  +headers
  +msg_body
}

class Method {
  <<enum>>
  GET
  POST
  DELETE
  Uninitialized
}

class Resource {
  <<enum>>
  Path(String)
}

class Response {
  <<struct>>
  -status_code
  -status_text
  -headers
  -body
  +new(status_code, headers, body) Response
  +send(write_stream) Result
  +status_code() string
  +status_text() string
  +headers() String
  +body() string
}

class Data {
  <<struct>>
  -id
  -date
  -status
}

Request ..|> From: Implements
Response ..|> From: Implements
Response ..|> Default: Implements
Method ..|> From: Implements

Server -- Router: Calls
Router -- Resource: Gets
Router -- Method: Checks
Router -- Request: Processes
Router .. Handler: Calls
Handler -- Request: Handles
Handler <|.. WebService: Implements
StaticPage ..|> Handler: Implements
ErrorPage ..|> Handler: Implements
Handler -- Response: Sends
Request *-- Resource: Belongs_to
Request *-- Method: Belongs_to
WebService -- Data: Loads

Request ..() Debug
Response ..() Debug
Response ..() PartialEq
Response ..() Clone
Method ..() Debug
Method ..() PartialEq
Resource ..() Debug
Resource ..() PartialEq
Data ..() Serialize
Data ..() Deserialize
Data ..* Response: Added_to
```

### Sequence

```mermaid
sequenceDiagram

Note left of Config: File
Config ->> Server: Initialisation
Server ->> Server: Bind Listener to Address

loop Listening...
  Client ->>+ Server: Request
  Server ->> Server: Status line
  Server ->> Server: HTML Page Content
  Server ->> Server: Content length
  Server -->>- Client: Response
  Note right of Client: View
end
```

## Contributors

### Authors

[![fakeita](https://shields.io/badge/Author-fakeita-magenta)](http://learn.zone01dakar.sn/git/fakeita)
[![jefaye](https://shields.io/badge/Author-jefaye-cyan)](http://learn.zone01dakar.sn/git/jefaye)

### Peers

[![npouille](https://shields.io/badge/Zone01-npouille-blue)](http://learn.zone01dakar.sn/git/npouille)

### Testers

### Auditors

## License

[![MIT](https://shields.io/badge/License-MIT-black)](LICENSE)
