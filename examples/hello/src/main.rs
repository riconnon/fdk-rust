extern crate fdk;

fn main() {
    if let Err(err) = fdk::handle() {
        eprintln!("error starting server: {}", err)
    }
}

fn myHandler() {
}
