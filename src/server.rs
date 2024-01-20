use crate::{request::Request, response::Response};

pub fn serve_file(request: Request) -> Response {
    let path = request.request_line.path;
}
