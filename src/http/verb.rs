use anyhow::{anyhow, Result};
use std::convert::TryFrom;
use std::fmt;

#[derive(Clone, Debug)]
pub enum HttpVerb {
    Connect,
    Delete,
    Get,
    Head,
    Options,
    Patch,
    Post,
    Put,
    Trace,
}

// so we can easily convert a raw string that we'll find in an HTTP request into an enum variant
impl TryFrom<&str> for HttpVerb {
    type Error = anyhow::Error;

    fn try_from(raw_verb: &str) -> Result<Self, anyhow::Error> {
        match raw_verb {
            "GET" => Ok(HttpVerb::Get),
            "PUT" => Ok(HttpVerb::Put),
            "TRACE" => Ok(HttpVerb::Trace),
            "POST" => Ok(HttpVerb::Post),
            "PATCH" => Ok(HttpVerb::Patch),
            "OPTIONS" => Ok(HttpVerb::Options),
            "HEAD" => Ok(HttpVerb::Head),
            "DELETE" => Ok(HttpVerb::Delete),
            "CONNECT" => Ok(HttpVerb::Connect),
            other => Err(anyhow!("Got an unexpected HTTP verb: {}", other)),
        }
    }
}

impl fmt::Display for HttpVerb {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HttpVerb::Get => write!(f, "GET"),
            HttpVerb::Put => write!(f, "PUT"),
            HttpVerb::Trace => write!(f, "TRACE"),
            HttpVerb::Post => write!(f, "POST"),
            HttpVerb::Patch => write!(f, "PATCH"),
            HttpVerb::Options => write!(f, "OPTIONS"),
            HttpVerb::Head => write!(f, "HEAD"),
            HttpVerb::Delete => write!(f, "DELETE"),
            HttpVerb::Connect => write!(f, "CONNECT"),
        }
    }
}
