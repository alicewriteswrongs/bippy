use crate::http::status::HttpStatus;
use crate::http::version::HttpVersion;

pub struct Response {
    pub version: HttpVersion,
    pub status: HttpStatus,
    pub body: String,
}

impl Response {
    fn format_status_line(&self) -> String {
        format!("{} {}", self.version, self.status)
    }

    pub fn format(&self) -> String {
        // TODO add headers
        format!("{}\r\n\r\n{}\r\n\r\n", self.format_status_line(), self.body)
    }
}
