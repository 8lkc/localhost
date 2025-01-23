<h1 align=center>
    localhost
    <br>
    <img alt="Ferris" src="assets/ferris.svg">
</h1>

## Table of Contents

- [Table of Contents](#table-of-contents)
- [Tech Stack](#tech-stack)
- [Overview](#overview)
  - [TCP Header](#tcp-header)
- [Installation](#installation)
  - [Cloning](#cloning)
  - [File System](#file-system)
- [Architecture](#architecture)
  - [Classes](#classes)
  - [Sequence](#sequence)
- [Usage](#usage)
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

### TCP Header

  ```mermaid
  ---
  title: "TCP Packet"
  ---
  packet-beta
  0-15: "Source Port"
  16-31: "Destination Port"
  32-63: "Sequence Number"
  64-95: "Acknowledgment Number"
  96-99: "Data Offset"
  100-105: "Reserved"
  106: "URG"
  107: "ACK"
  108: "PSH"
  109: "RST"
  110: "SYN"
  111: "FIN"
  112-127: "Window"
  128-143: "Checksum"
  144-159: "Urgent Pointer"
  160-191: "(Options and Padding)"
  192-255: "Data (variable length)"
```

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
```
📂./
  |
  +-📂 /assets
  |       |
  |       +-🌄 ferris.svg
  |
  +-📂 /config
  |       |
  |       +-⚙️ server.toml
  |
  +-📂 /data
  |       |
  |       +-📄 data.json
  |
  +-📂 /templates
  |       |
  |       +-📄 error.html
  |       +-📄 index.html
  |
  +-📂 /scripts
  |       |
  |       +-📜 gitify.sh
  |       +-📜 utils.sh
  |
  +---📂 /src
  |       |
  |       +--📂 /http
  |       |       |
  |       |       +-📂 /request
  |       |       |       |
  |       |       |       +-📄 method.rs
  |       |       |       +-📄 mod.rs
  |       |       |       +-📄 utils.rs
  |       |       |
  |       |       +-📂 /response
  |       |       |       |
  |       |       |       +-📄 func.rs
  |       |       |       +-📄 mod.rs
  |       |       |
  |       |       +-📄 mod.rs
  |       |
  |       +-📂 /server
  |       |       |
  |       |       +-📂 /handler
  |       |       |       |
  |       |       |       +-📄 mod.rs
  |       |       |       +-📄 static_page.rs
  |       |       |       +-📄 web_service.rs
  |       |       |
  |       |       +-📄 mod.rs
  |       |       +-📄 router.rs
  |       |
  |       +-📄 lib.rs
  |       +-📄 loader.rs
  |       +-📄 main.rs
  |       +-📄 mux.rs
  |
  +-📂 /tests
  |       |
  |       +-📄 request_test.rs
  |       +-📄 response_test.rs
  |
  +-📂 /todos
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
```

## Architecture

```mermaid
architecture-beta

  group localhost(logos:rust)[localhost]
  group source(logos:rust)[source] in localhost
  group servers(logos:rust)[servers] in source
  group http(logos:rust)[http] in source

  service config(logos:toml)[config] in localhost
  service templates(logos:html-5)[templates] in localhost
  service data(logos:json)[data] in localhost

  service loader(logos:aws-config)[loader] in source
  service multiplexer(server)[multiplexer] in source

  service root(server)[root] in  servers
  service middlewares(logos:aws-lambda)[middlewares] in servers
  service router(logos:react-router)[router] in servers
  service handlers(logos:aws-step-functions)[handlers] in servers

  service requests(internet)[requests] in http
  service sessions(internet)[sessions] in http
  service responses(internet)[responses] in http

  junction builder in localhost

  config:R --> L:loader
  loader:R --> L:multiplexer
  requests:L --> R:multiplexer
  multiplexer:B --> T:root
  root:R <-- L:sessions
  root:L --> R:middlewares
  middlewares:B --> T:router
  router:R --> L:handlers
  builder:T --> B:handlers
  templates:L -- R:builder
  data:R -- L:builder
  handlers:R --> L:responses
```

### Classes

```mermaid
classDiagram
%% direction LR

class From {
  <<trait>>
  +from(str) Self
  +from(Response) String
}

class Default {
  <<trait>>
  +default() Self
}

class Multiplexer {
  <<struct>>
  +servers
  +default()
  +clean()
}

class Loader
<<struct>> Loader

namespace server {
  class Handler {
    <<trait>>
    +handle(request) Response
    +load_file(file_name) String
  }

  class Server {
    <<struct>>
    -host
    -ports
    -methods
    -timeout
    +run() Result
    +has_valid_config() Result
    +host() String
    +ports() [usize]
    +methods() [String]
    +timeout() usize
  }

  class Router {
    <<struct>>
    +route(req, stream) 
  }

  class WebService {
    <<struct>>
    +load_json() [Data]
  }

  class StaticPage {
    <<struct>>
  }

  class ErrorPage {
    <<struct>>
  }
}

namespace http {
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
}

class Data {
  <<struct>>
  -id
  -date
  -status
}

Multiplexer ..|> Default: Implements
Request ..|> From: Implements
Response ..|> From: Implements
Response ..|> Default: Implements
Method ..|> From: Implements

Loader -- Multiplexer: Configures
Loader -- Server: Configures
Multiplexer *-- Server: Has
Server -- Request: Builds
Server -- Router: Calls
Router -- Request: Directs
Router -- Resource: Gets
Router -- Method: Checks
Router .. Handler: Calls
Handler -- Request: Handles
Handler <|.. WebService: Implements
StaticPage ..|> Handler: Implements
ErrorPage ..|> Handler: Implements
Handler -- Response: Sends
Request *-- Resource: Belongs_to
Request *-- Method: Belongs_to
WebService -- Data: Loads
Data ..* Response: Added_to

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
```

### Sequence

```mermaid
sequenceDiagram
title TCP Connection
  participant Client
  participant Server

  Note over Client,Server: Sequence numbers is relative.<br/>It's usually a random number.

  activate Client
  Client->>+Server: TCP SYN Seq=0
  Server-->>Client: TCP SYN-ACK Seq=0 Ack=1
  Client-->>Server: TCP ACK Seq=1 Ack=1

  Note over Client,Server: Connected
  loop
    Client->>Server: Data Seq=1 Ack=1 
    Server-->>Client: Data Seq=1 Ack=2 
  end
  Note over Client,Server: Disconnection...

  Client->>Server: TCP FIN Seq=2 Ack=1
  Server-->>Client: TCP ACK Seq=1 Ack=3
  Server->>Client: TCP FIN Seq=1 Ack=3
  Client-->>Server: TCP ACK Seq=2 Ack=2
  deactivate Server
  deactivate Client
  Note over Client,Server: Disconnected
```

## Usage

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
