use super::server::Handler;
use super::http::{Request, Response, StatusCode, Method};
use std::fs;

pub struct WebSiteHandler {
    public_path: String
}

impl WebSiteHandler {
    pub fn new(public_path: String) -> Self {
        Self {public_path}
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);
        fs::read_to_string(path).ok()
    }
}

impl Handler for WebSiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        Response::new(StatusCode::Ok, Some("<h1> TEST </h1>".to_string()));
            match request.method() {
                Method::GET => match request.path() {
                    "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                    "/ahoy" => Response::new(StatusCode::Ok, Some("<h1>Ahoy there!</h1>".to_string())),
                    "/home" => Response::new(StatusCode::Ok, Some("<h1>Welcome home!</h1>".to_string())),
                    _ =>Response::new(StatusCode::NotFound, None),
                }
                _ =>Response::new(StatusCode::NotFound, None),
            }
    }
}