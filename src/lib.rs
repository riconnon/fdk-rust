extern crate hyper;
extern crate hyperlocal;
extern crate futures;

use self::hyper::{header, Body, Request, Response};
use hyper::service::service_fn;
use std::env;
use std::fs;
use std::io;

const FN_LISTENER: &str = "FN_LISTENER";
const PHRASE: &str = "Hello, World!";

fn hello(r: impl io::BufRead, mut w: impl io::Write) -> io::Result<()> {
    if let Err(err) = w.write(PHRASE.as_bytes()) {
        return Err(err)
    }
    Ok(())
}

fn fnHandler(f: &Fn() -> io::Result<()>) -> (Fn(Request<Body>) -> io::Result<()>) {
    fn handler(_: Request<Body>) -> impl futures::Future<Item = Response<Body>, Error = io::Error> + Send {
        futures::future::ok(
            Response::builder()
                .header(header::CONTENT_TYPE, "text/plain")
                .header(header::CONTENT_LENGTH, PHRASE.len() as u64)
                .body(PHRASE.into())
                .expect("failed to create response")
        )
    }
}

pub fn handle() -> io::Result<()> {
    let path = env::var(FN_LISTENER).unwrap();
    if let Err(err) = fs::remove_file(&path) {
        if err.kind() != io::ErrorKind::NotFound {
            return Err(err)
        }
    }
    let svr = hyperlocal::server::Server::bind(&path, || service_fn(fnHandler(hello)))?;
    println!("Listening on unix://{path} with 1 thread.", path = &path);
    svr.run()?;
    Ok(())
}
