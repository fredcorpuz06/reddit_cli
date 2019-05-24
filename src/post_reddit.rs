// Send POST Requests to Reddit API
// Code not used because it is incomplete without being able to perform
// OAuth2 for Reddit API

extern crate hyper;
use std::io::{self, Write};
use hyper::Client;
use hyper::header::{HeaderValue, CONTENT_TYPE};
use hyper::rt::{self, Future, Stream};
use hyper::{Method, Request, Body};

fn main() {
    rt::run(rt::lazy(|| {
        let https = HttpsConnector::new(4).expect("TLS initialization failed");
        let client = Client::builder()
            .build::<_, hyper::Body>(https);

        // GET Request
        let get = client.get("http://httpbin.org/headers".parse().unwrap()).and_then(|res| {
            // println!("GET: {}", res.status());
            res.into_body().concat2()
        });

        let json = r#"{"library":"hyper"}"#;
        let uri: hyper::Uri = "https://reddit.com/api/comment".parse().unwrap();
        let mut req = Request::new(Body::from(json));
        *req.method_mut() = Method::POST;
        *req.uri_mut() = uri.clone();
        req.headers_mut().insert(
            CONTENT_TYPE, HeaderValue::from_static("application/json")
        );


        // POST Request
        let post = client.request(req).and_then(|res| {
            // println!("POST: {}", res.status());
            res.into_body().concat2()
        });

        // Run both futures together
        let work = post.join(get);
        work
            .map(|(posted, got)| {
                println!("GET: {:?}", got);
                println!("POST: {:?}", posted);
            })
            .map_err(|err| {
                println!("Error: {}", err);
            })
    }));
}