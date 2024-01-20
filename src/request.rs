use crate::http::{verb::HttpVerb, version::HttpVersion};
use std::fmt;

#[derive(Debug)]
pub struct Request {
    pub verb: HttpVerb,
    pub path: String,
    pub version: HttpVersion,
}

impl fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.verb, self.path, self.version)
    }
}
