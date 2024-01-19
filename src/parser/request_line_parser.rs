use crate::{
    http_verb::HttpVerb, http_version::HttpVersion, request::RequestLine,
};
use anyhow::{anyhow, Result};

use std::{collections::VecDeque, convert::TryFrom};

impl TryFrom<&mut HttpRequestLineParser<'_>> for RequestLine {
    type Error = anyhow::Error;

    fn try_from(
        parser: &mut HttpRequestLineParser,
    ) -> Result<Self, anyhow::Error> {
        if parser.state != HttpRequestLineParserState::Done {
            return Err(anyhow!(
                "tried to construct a RequestLine struct before finishing parsing!"
            ));
        }

        // borrow checker shenanigans
        let parser = parser.clone();

        let request_line = RequestLine {
            verb: parser.verb.expect("HTTP verb should be present").clone(),
            path: parser.path.expect("HTTP path should be present"),
            version: parser.version.expect("HTTP version should be present"),
        };
        Ok(request_line)
    }
}

#[derive(PartialEq, Clone)]
enum HttpRequestLineParserState {
    ParsingVerb,
    ParsingPath,
    ParsingVersion,
    Done,
}

#[derive(Clone)]
struct HttpRequestLineParser<'a> {
    raw_request_line: VecDeque<&'a str>,
    state: HttpRequestLineParserState,
    verb: Option<HttpVerb>,
    path: Option<String>,
    version: Option<HttpVersion>,
}

impl HttpRequestLineParser<'_> {
    pub fn new(raw_request: &str) -> HttpRequestLineParser<'_> {
        let words = raw_request.split(' ').collect::<Vec<&str>>();
        HttpRequestLineParser {
            raw_request_line: VecDeque::from(words),
            state: HttpRequestLineParserState::ParsingVerb,
            verb: None,
            path: None,
            version: None,
        }
    }

    pub fn parse_word(&mut self) {
        if let Some(raw_word) = self.raw_request_line.pop_front() {
            match self.state {
                HttpRequestLineParserState::ParsingVerb => {
                    let verb = HttpVerb::try_from(raw_word)
                        .expect("should be able to convert raw string to verb");
                    self.verb = Some(verb);
                    self.state = HttpRequestLineParserState::ParsingPath;
                }
                HttpRequestLineParserState::ParsingPath => {
                    // TODO add some validation that this is "path-like" here
                    self.path = Some(raw_word.to_string());
                    self.state = HttpRequestLineParserState::ParsingVersion;
                }
                HttpRequestLineParserState::ParsingVersion => {
                    self.version = Some(HttpVersion::OneDotOne);
                    self.state = HttpRequestLineParserState::Done;
                }
                HttpRequestLineParserState::Done => {}
            }
        } else {
            panic!("couldn't find a word when I expected one!");
        }
    }

    pub fn parse(&mut self) -> Result<RequestLine> {
        while self.state != HttpRequestLineParserState::Done {
            self.parse_word();
        }
        RequestLine::try_from(self)
    }
}

pub fn parse(line: &str) -> Result<RequestLine> {
    let mut parser = HttpRequestLineParser::new(line);
    parser.parse()
}
