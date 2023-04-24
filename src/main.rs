use cgi::{http::StatusCode, Request, Response};
mod counter;

fn main() {
    cgi::handle(handler);
}

fn handler(_request: Request) -> Response {
    // TODO: handle error
    let file_path = "/tmp/counter/counter.txt";
    let pageviews = match counter::increment_counter(file_path) {
        Ok(pageviews) => pageviews,
        Err(e) => {
            eprintln!("error: {e}");
            return cgi::string_response(StatusCode::INTERNAL_SERVER_ERROR, format!("{{\"error\": \"{e}\"}}"))
        }
    };

    cgi::string_response(StatusCode::OK, format!("{{\"pageviews\": {pageviews}}}"))
}
