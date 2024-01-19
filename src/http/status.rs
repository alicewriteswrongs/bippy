use std::fmt;

pub enum HttpStatus {
    Ok200,
    NotFound404,
}

impl fmt::Display for HttpStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HttpStatus::Ok200 => write!(f, "200 OK"),
            HttpStatus::NotFound404 => write!(f, "404 NOT FOUND"),
        }
    }
}
