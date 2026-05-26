mod conversion;
mod protocol;
mod server;
mod uri;

fn main() {
    if let Err(error) = server::run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
