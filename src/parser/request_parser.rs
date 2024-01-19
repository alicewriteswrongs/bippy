use std::collections::VecDeque;

use anyhow::Result;

use super::request_line_parser;
use crate::request::Request;

pub fn parse_request(raw_request: &Vec<String>) -> Result<Request> {
    let mut lines: VecDeque<String> = VecDeque::from(raw_request.to_owned());

    let first_line = lines
        .pop_front()
        .expect("Expected to find a request line but found none");

    let request_line = request_line_parser::parse(&first_line)?;

    Ok(Request { request_line })
}
