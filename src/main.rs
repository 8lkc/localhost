use localhost::Loader;

/// Performs the following steps:
/// 1. Loading the server configuration from the specified TOML file.
/// 2. Initializing a `Multiplexer` with the loaded configuration.
/// 3. Adding the listening socket to the `Multiplexer`'s event loop.
/// 4. Starting the event loop, handling incoming connections and processing requests.
/// 
/// # Errors
/// 
/// Throws an error if:
/// * The server configuration cannot be loaded from the file.
/// * An error occurs while adding the listening socket to the `Multiplexer`.
/// 
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
