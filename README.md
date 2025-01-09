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
        |       +-📂 config/
        |       |       |
        |       |       +-📄 mod.rs
        |       |       +-📄 parser.rs
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
        |       |       +-📄 connection.rs
        |       |       +-📄 epoll.rs
        |       |       +-📄 mod.rs
        |       |
        |       +-📄 lib.rs
        |       +-📄 main.rs
        |
        +-📂 todos/
        |       |
        |       +-📝 audit.todo
        |       +-📝 tasks.todo
        |
        +-🚫 .gitignore
        +-🔒 Cargo.lock
        +-⚙️ Cargo.toml
        +-🔑 LICENSE
        +-📖 README.md
        +-⚙️ rustfmt.toml


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
