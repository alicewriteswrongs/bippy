use crate::http_verb::HttpVerb;
use crate::http_version::HttpVersion;

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
