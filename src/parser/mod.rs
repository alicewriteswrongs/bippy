use std::collections::VecDeque;

mod request;
mod request_line_parser;

pub use request::Request;

pub fn parse_request(raw_request: Vec<String>) -> anyhow::Result<Request> {
    let lines: VecDeque<String> = raw_request.into();

    let request_line = request_line_parser::parse(lines.pop_front().expect("Expected to find a request line but found none"))?;

    Ok(Request {
        request_line
    })
}
