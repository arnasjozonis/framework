extern crate hyper;

use std::io::{self, Write};
use hyper::Client;
use hyper::rt::{self, Future, Stream};

pub struct RestClient {
}

impl RestClient {
    pub fn start() -> (){
        rt::run(rt::lazy(|| {
            let client = Client::new();
            let uri = "http://localhost:5052/beacon/validators".parse().unwrap();

            client
            .get(uri)
            .and_then(|res| {
                println!("Response: {}", res.status());
                println!("Headers: {:#?}", res.headers());

                res.into_body().for_each(|chunk| {
                    io::stdout().write_all(&chunk)
                        .map_err(|e| panic!("example expects stdout is open, error={}", e))
                })
            })
            .map(|_| {
                println!("\n\nDone.");
            })
            .map_err(|err| {
                eprintln!("Error {}", err);
            })
        }));
    }
}