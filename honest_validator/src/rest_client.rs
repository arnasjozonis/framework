// TODO: rewrite rest client with reqwest: https://crates.io/crates/reqwest
// Hyper Imports
use hyper::client::Client;
use hyper::header::HeaderValue;
use hyper::{self, Body, Method, Request, Uri};
type HttpConnector = hyper::client::HttpConnector;

use futures::future::{ok, Future};
use futures::stream::Stream;
use tokio_core::reactor::Core;

use serde::de::DeserializeOwned;
use serde::{Serialize};

use std::cell::RefCell;
use std::rc::Rc;
use std::str;

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
    pub fn new(base_url: String) -> Option<RestClient> {
        let core = Core::new().unwrap();
        let http = Client::builder().build(HttpConnector::new(4));
        Some(RestClient {
            core: Rc::new(RefCell::new(core)),
            http: Rc::new(http),
            base_url,
        })
    }

    pub fn post<TResult, TBody>(&self, resource_uri: &str, body: Option<TBody>) -> Option<TResult>
    where
        TResult: DeserializeOwned + Default,
        TBody: Serialize
    {
        let host = self.base_url.clone();
        let uri: Uri = (host + resource_uri).parse().unwrap();
        println!("Calling POST: {}", uri);
        match self.post_request(uri, body) {
            Some(response) => response,
            None => Some(TResult::default())
        }
    }

    fn post_request<TResult, TBody>(&self, resource_uri: Uri, body: Option<TBody>) -> Option<TResult>
    where
        TResult: DeserializeOwned + Default,
        TBody: Serialize
    {
        let mut core_ref = self
            .core
            .try_borrow_mut()
            .unwrap();
        let client = &self.http;
        
        let req_body = match body {
            Some(b) => {
                let json = serde_json::to_string(&b).unwrap();
                Body::from(json)
            },
            None => Body::empty()
        };

        let mut req = Request::new(req_body);
        *req.method_mut() = Method::POST;
        *req.uri_mut() = resource_uri.clone();
        req.headers_mut().insert(
            hyper::header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );

        let work = client.request(req)
            .and_then(|res| {
                res.into_body()
                    .fold(Vec::new(), |mut v, chunk| {
                        v.extend(&chunk[..]);
                        ok::<_, hyper::Error>(v)
                    })
                    .map(move |chunks| {
                        if chunks.is_empty() {
                            None
                        } else {
                            let result = match serde_json::from_slice(&chunks) {
                                Ok(res) => res,
                                Err(e) => { println!("Error in parsing response json: {}", e); Some(TResult::default())}
                            };
                            result
                        }
                    })
            });

        match core_ref.run(work) {
            Ok(future_item) => future_item,
            Err(e) => { println!("API error: {}", e); None }
        }
    }

    pub fn get<TResult>(&self, resource_uri: &str) -> Option<TResult>
    where
        TResult: DeserializeOwned,
    {
        let host = self.base_url.clone();
        let uri: Uri = (host + resource_uri).parse().unwrap();
        println!("{}", uri);
        self.get_request(uri)
    }

    fn get_request<TResult>(&self, uri: Uri) -> Option<TResult>
    where
        TResult: DeserializeOwned,
    {
        let mut core_ref = self
            .core
            .try_borrow_mut()
            .unwrap();
        let client = &self.http;
        let mut req = Request::new(Body::empty());

        *req.method_mut() = Method::GET;
        *req.uri_mut() = uri.clone();
        req.headers_mut().insert(
            hyper::header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );
        let work = client.request(req)
            .and_then(|res| {
                res.into_body()
                    .fold(Vec::new(), |mut v, chunk| {
                        v.extend(&chunk[..]);
                        ok::<_, hyper::Error>(v)
                    })
                    .map(move |chunks| {
                        if chunks.is_empty() {
                            None
                        } else {
                            let result = match serde_json::from_slice(&chunks) {
                                Ok(res) => res,
                                Err(e) => { println!("Error in parsing response json: {}", e); None}
                            };
                            result
                        }
                    })
            });

        match core_ref.run(work) {
            Ok(future_item) => future_item,
            Err(e) => { println!("API error: {}", e); None }
        }
    }
}
