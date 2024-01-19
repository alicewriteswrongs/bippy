use anyhow::{anyhow, Result};
use std::convert::TryFrom;

pub enum HttpVerb {
    CONNECT,
    DELETE,
    GET,
    HEAD,
    OPTIONS,
    PATCH,
    POST,
    PUT,
    TRACE,
}

// so we can easily convert a raw string that we'll find in an HTTP request into an enum variant
impl TryFrom<&String> for HttpVerb {
    type Error = anyhow::Error;

    fn try_from(raw_verb: &String) -> Result<Self, anyhow::Error> {
        match raw_verb.as_str() {
            "GET" => Ok(HttpVerb::GET) ,
            "PUT" => Ok(HttpVerb::PUT) ,
            "TRACE" => Ok(HttpVerb::TRACE) ,
            "POST" => Ok(HttpVerb::POST) ,
            "PATCH" => Ok(HttpVerb::PATCH) ,
            "OPTIONS" =>Ok(HttpVerb::OPTIONS) ,
            "HEAD" => Ok(HttpVerb::HEAD) ,
            "DELETE" =>Ok(HttpVerb::DELETE) ,
            "CONNECT" =>Ok(HttpVerb::CONNECT) ,
            other => Err(anyhow!("Got an unexpected HTTP verb: {}", other)),
        }
    }
}
