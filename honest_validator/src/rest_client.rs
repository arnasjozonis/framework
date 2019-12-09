// Hyper Imports
use hyper::client::Client;
use hyper::header::HeaderValue;
use hyper::StatusCode;
use hyper::{self, Body, Method, Request, Uri};
type HttpConnector = hyper::client::HttpConnector;

use futures::future::{ok, Future};
use futures::stream::Stream;
use tokio_core::reactor::Core;

use serde::de::DeserializeOwned;
use serde::Deserialize;

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

    pub fn post<TResult>(&mut self, resource_uri: &str) -> Option<TResult>
    where
        TResult: DeserializeOwned,
    {
        let host = self.base_url.clone();
        let uri: Uri = (host + resource_uri).parse().unwrap();
        println!("{}", uri);
        self.post_request(uri).unwrap()
    }

    fn post_request<TResult>(&mut self, uri: Uri) -> Option<TResult>
    where
        TResult: DeserializeOwned,
    {
        let mut core_ref = self
            .core
            .try_borrow_mut()
            .unwrap();
        let client = &self.http;

        let json = r#"{"library":"hyper"}"#;

        let mut req = Request::new(Body::from(json));
        *req.method_mut() = Method::POST;
        *req.uri_mut() = uri.clone();
        req.headers_mut().insert(
            hyper::header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );

        let work = client.request(req).and_then(|res| {
            res.into_body()
                .fold(Vec::new(), |mut v, chunk| {
                    v.extend(&chunk[..]);
                    ok::<_, hyper::Error>(v)
                })
                .map(move |chunks| {
                    if chunks.is_empty() {
                        None
                    } else {
                        Some(serde_json::from_slice(&chunks).unwrap())
                    }
                })
        });
        core_ref
            .run(work).unwrap().unwrap()
    }

    pub fn get<TResult>(&mut self, resource_uri: &str) -> Option<TResult>
    where
        TResult: DeserializeOwned,
    {
        let host = self.base_url.clone();
        let uri: Uri = (host + resource_uri).parse().unwrap();
        println!("{}", uri);
        self.get_request(uri).unwrap()
    }

    fn get_request<TResult>(&mut self, uri: Uri) -> Option<TResult>
    where
        TResult: DeserializeOwned,
    {
        let mut core_ref = self
            .core
            .try_borrow_mut()
            .unwrap();
        let client = &self.http;
        let work = client.get(uri).and_then(|res| {
            res.into_body()
                .fold(Vec::new(), |mut v, chunk| {
                    v.extend(&chunk[..]);
                    ok::<_, hyper::Error>(v)
                })
                .map(move |chunks| {
                    if chunks.is_empty() {
                        None
                    } else {
                        Some(serde_json::from_slice(&chunks).unwrap())
                    }
                })
        });
        core_ref
            .run(work).unwrap().unwrap()
    }
}
