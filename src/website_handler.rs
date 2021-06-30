use super::server::Handler;
use super::http::{Request, Response, StatusCode, Method};

pub struct WebSiteHandler {
    public_path: String
}

impl WebSiteHandler {
    pub fn new(public_path: String) -> Self {
        Self {public_path}
    }
}

impl Handler for WebSiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        Response::new(StatusCode::Ok, Some("<h1> TEST </h1>".to_string()));
            match request.method() {
                Method::GET => match request.path() {
                    "/" => Response::new(StatusCode::Ok, Some("<h1>Hello world!</h1>".to_string())),
                    "/ahoy" => Response::new(StatusCode::Ok, Some("<h1>Ahoy there!</h1>".to_string())),
                    "/home" => Response::new(StatusCode::Ok, Some("<h1>Welcome home!</h1>".to_string())),
                    _ =>Response::new(StatusCode::NotFound, None),
                }
                _ =>Response::new(StatusCode::NotFound, None),
            }
    }
}