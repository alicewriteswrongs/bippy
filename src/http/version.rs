use std::fmt;

#[derive(Clone, Debug)]
pub enum HttpVersion {
    OneDotOne,
}

impl fmt::Display for HttpVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "HTTP/1.1")
    }
}
