fn main() {
    
    // let string = String::from("127.0.0.1:8080");
    // let string_slice = &string[10..];
    // dbg!(&string);
    // dbg!(string_slice);

    // let get = Method::GET("abcd".to_string());
    // let delete = Method::DELETE(100);
    // let post = Method::POST;
    // let put = Method::PUT;
    
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}

struct Server {
    addr: String,

}

impl Server {
    fn new(addr: String) -> Self {
        Self {
            addr 
        }
    }
    fn run(self) {
        println!("Listening on {}", self.addr);

    }
}

struct Request {
    path: String, 
    query_string: String,
    method: String,
}

enum Method {
    GET,
    DELETE,
    POST,
    PUT,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATH,
}


// GET /user?id=10 HTTP/1.1\r\n
// HEADERS \r\n
// BODY
