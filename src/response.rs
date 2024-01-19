use crate::http::status::HttpStatus;
use crate::http::version::HttpVersion;

pub struct Response {
    version: HttpVersion,
    status: HttpStatus,
    body: String,
}
