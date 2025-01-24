use localhost::Loader;

fn main() {
    let mux = match Loader::load("./config/server.toml") {
        Ok(multiplexer) => multiplexer,
        Err(error) => {
            dbg!(error);
            return;
        }
    };

    if let Err(error) = mux.add_fd() {
        dbg!(error);
    }

    dbg!(mux.epoll_fd());

    mux.run()
}
