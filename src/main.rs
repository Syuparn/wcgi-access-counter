use cgi::{http::StatusCode, Request, Response};
mod counter;

fn main() {
    cgi::handle(handler);
}

fn handler(_request: Request) -> Response {
    // TODO: handle error
    let file_path = "/tmp/counter.txt";
    let cnt_accesses = counter::increment_counter(file_path).unwrap();

    cgi::text_response(StatusCode::OK, format!("{{\"accesses\": {cnt_accesses}}}"))
}
