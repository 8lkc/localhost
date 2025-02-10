use server::Localhost;

mod server;

fn main() {
    let localhost = Localhost::new();
    localhost.start();
}
