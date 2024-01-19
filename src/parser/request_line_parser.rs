use anyhow::{anyhow, Result};
use crate::http_verb::HttpVerb;

use std::{convert::TryFrom, collections::VecDeque};

enum HttpVersion {
    OneDotOne,
}

pub struct RequestLine {
    verb: HttpVerb,
    path: String,
    version: HttpVersion,
}

impl TryFrom<HttpRequestLineParser> for RequestLine {
    type Error = anyhow::Error;

    fn try_from(parser: HttpRequestLineParser) -> Result<Self, anyhow::Error> {
        if parser.state != HttpRequestLineParserState::Done {
            return Err(anyhow!(
                "tried to construct a RequestLine struct before finishing parsing!"
            ));
        }

        let request_line = RequestLine {
            verb: parser.verb.expect("HTTP verb should be present"),
            path: parser.path.expect("HTTP path should be present"),
            version: parser.version.expect("HTTP version should be present"),
        };
        Ok(request_line)
    }
}

#[derive(PartialEq)]
enum HttpRequestLineParserState {
    Init,
    ParsingVerb,
    ParsingPath,
    ParsingVersion,
    Done,
}

struct HttpRequestLineParser {
    raw_request_line: VecDeque<String>,
    state: HttpRequestLineParserState,
    verb: Option<HttpVerb>,
    path: Option<String>,
    version: Option<HttpVersion>,
}

impl HttpRequestLineParser {
    pub fn new(raw_request: String) -> HttpRequestLineParser {
        HttpRequestLineParser {
            raw_request_line: raw_request.split(' ').into(),
            state: HttpRequestLineParserState::Init,
            verb: None,
            path: None,
            version: None,
        }
    }

    pub fn chomp_word(&mut self) -> Result<String> {
        if self.raw_request_line.len() == 0 {
            return Err(anyhow!("tried to parse a request but found no more lines!"));
        }

        let head = self.raw_request_line.get(0).expect("we should always have a first element here");
    }



    pub fn parse(&mut self) {
        match self.state {
            HttpRequestLineParserState::Init => {
                if self.raw_request_line.is_empty() {
                    panic!("found an empty line when starting to parse the request line!");
                }
                self.state = HttpRequestLineParserState::ParsingVerb;
            },
            HttpRequestLineParserState::ParsingVerb => {
                let raw_verb = self.raw_request_line.pop_front().expect("should have a verb candidate");
                let verb = HttpVerb::try_from(&raw_verb)?;

                self.verb = Some(verb);
            },
            HttpRequestLineParserState::ParsingPath => {
            },
            HttpRequestLineParserState::ParsingVersion => {
            },
            HttpRequestLineParserState::Done => {}
        }
    }
}


