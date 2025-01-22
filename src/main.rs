use localhost::Loader;

fn main() {
    let mux = Loader::load("./config/server.toml").unwrap_or_default();

    for server in mux.servers {
        println!("{:#?}", server);

        if let Err(error) = server.run() {
            println!("{error}");
            continue;
        }
    }
}
