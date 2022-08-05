use http::Request;
use http::Method;
use http::Server;

mod http;


fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}



/*

GET /user?id=10 HTTP/1.1\r\n
HEADERS \r\n
BODY


*/
