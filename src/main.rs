use cgi::{http::StatusCode, Request, Response};
mod counter;

fn main() {
    cgi::handle(handler);
}

fn handler(_request: Request) -> Response {
    // TODO: handle error
    let file_path = "/tmp/counter.txt";
    let cnt_accesses = match counter::increment_counter(file_path) {
        Ok(cnt_access) => cnt_access,
        Err(e) => {
            eprintln!("error: {e}");
            return cgi::string_response(StatusCode::INTERNAL_SERVER_ERROR, format!("{{\"error\": \"{e}\"}}"))
        }
    };

    cgi::string_response(StatusCode::OK, format!("{{\"accesses\": {cnt_accesses}}}"))
}
