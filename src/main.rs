use cgi::{http::StatusCode, Request, Response};
mod counter;

const COUNTER_FILE_PATH: &str = "/tmp/counter/counter.txt";

fn main() {
    cgi::handle(handler);
}

fn handler(_request: Request) -> Response {
    let pageviews = match counter::increment_counter(COUNTER_FILE_PATH) {
        Ok(pageviews) => pageviews,
        Err(e) => {
            eprintln!("error: {e}");
            return cgi::string_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("{{\"error\": \"{e}\"}}"),
            );
        }
    };

    cgi::string_response(StatusCode::OK, format!("{{\"pageviews\": {pageviews}}}"))
}
