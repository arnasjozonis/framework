// Hyper Imports
use hyper::client::Client;
use hyper::{self, Uri};
type HttpConnector = hyper::client::HttpConnector;

use futures::future::{ ok, Future };
use futures::stream::{Stream};
use tokio_core::reactor::Core;

use serde::de::DeserializeOwned;
use serde::Deserialize;

use std::cell::RefCell;
use std::rc::Rc;
use std::str;

use crate::errors::*;

pub struct RestClient {
    base_url: String,
    http: Rc<Client<HttpConnector>>,
    core: Rc<RefCell<Core>>,
}

impl Clone for RestClient {
    fn clone(&self) -> Self {
        Self {
            base_url: self.base_url.clone(),
            core: Rc::clone(&self.core),
            http: Rc::clone(&self.http),
        }
    }
}

impl RestClient {

    pub fn new(base_url: String) -> Result<RestClient> {

        let core = Core::new()?;
        let http = Client::builder().build(HttpConnector::new(4));
        Ok(RestClient {
            core: Rc::new(RefCell::new(core)),
            http: Rc::new(http),
            base_url
        })

    }
    pub fn get<TResult>(&mut self, resource_uri: &str) -> Option<TResult> 
    where TResult: DeserializeOwned
    {
        let host = self.base_url.clone();
        let uri: Uri = (host + resource_uri).parse().unwrap();
        println!("{}", uri);
        self.get_request(uri).unwrap()
    }

    fn get_request<TResult>(&mut self, uri: Uri) -> Result<Option<TResult>>
    where TResult: DeserializeOwned
    {
        let mut core_ref = self.core.try_borrow_mut().chain_err(|| {
            "Error when trying to burrow mutable runtime core"
        })?;
        let client = &self.http;
        let work = client
            .get(uri)
            .and_then(|res| {
                res.into_body()
                    .fold(Vec::new(), |mut v, chunk| {
                        v.extend(&chunk[..]);
                        ok::<_, hyper::Error>(v)
                    })
                    .map(move |chunks| {
                        if chunks.is_empty() {
                            Ok(None)
                        } else {
                            Ok(
                                Some(
                                    serde_json::from_slice(&chunks)
                                        .chain_err(|| "Failed to parse response body")?,
                                ),
                            )
                        }
                    })
            });
        core_ref
            .run(work)
            .chain_err(|| "Failed to execute request")?
    }
}