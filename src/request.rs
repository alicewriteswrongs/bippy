use crate::http::{verb::HttpVerb, version::HttpVersion};
use std::fmt;

#[derive(Debug)]
pub struct Request {
    pub request_line: RequestLine,
}

#[derive(Debug)]
pub struct RequestLine {
    pub verb: HttpVerb,
    pub path: String,
    pub version: HttpVersion,
}

impl fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.request_line.verb,
            self.request_line.path,
            self.request_line.version
        )
    }
}
