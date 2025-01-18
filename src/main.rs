use {
    localhost::Server,
    std::io::Result,
};

fn main() -> Result<()> { Server::new("127.0.0.1:7878").run() }
