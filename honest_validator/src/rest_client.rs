extern crate hyper;
extern crate serde;
extern crate serde_json;

use std::io::{self, Write};
use hyper::{Client, Error};
use hyper::rt::{self, Future, Stream};
use hyper::body::{Payload, Body };
use std::str;
use serde::{Deserialize, Serialize};
use serde_json::{Result, from_str, Error as SerializeError};

#[derive(Serialize, Deserialize)]
struct Validator {
    pubkey: String,
    withdrawal_credentials: String,
    effective_balance:u128,
    slashed:bool,
    activation_eligibility_epoch:u64,
    activation_epoch:u64,
    exit_epoch:u128,
    withdrawable_epoch:u128
}

pub struct RestClient {
}

impl RestClient {
    pub fn start() -> (){
        rt::run(rt::lazy(|| {
            let client = Client::new();
            let uri = "http://localhost:5052/beacon/validators".parse().unwrap();

            client
                .get(uri)
                .and_then(|res| res.into_body().concat2())
                .and_then(|c| {
                    Ok(
                        str::from_utf8(&c)
                        .map(|res| {
                            let r: Vec<Validator> =  match serde_json::from_str(res){
                                Ok(r) => r,
                                _ => Vec::new()
                            };
                            println!("{}", r.first().unwrap().pubkey);
                            res.to_owned()
                        })
                        .expect("The body could not be parsed as a UTF-8 string !")
                    )
                })
                .map(|_| {
                    println!("\n\nDone.");
                })
                .map_err(|err| {
                    eprintln!("Error {}", err);
                })
            })
        )
    }
}