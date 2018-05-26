extern crate gotham;
extern crate hyper;
extern crate rusty_wiki;

use gotham::router::Router;
use gotham::router::builder::*;

use rusty_wiki::handlers::hello_handler::say_hello;

fn router() -> Router {
    build_simple_router(|route| {
        route.get("/posts").to(say_hello);
    })
}

pub fn main() {
    let addr = "127.0.0.1:7878";
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, router())
}

#[cfg(test)]
mod tests {
    use super::*;
    use gotham::test::TestServer;
    use hyper::StatusCode;

    #[test]
    fn receive_hello_world_response() {
        let test_server = TestServer::new(router()).unwrap();
        let response = test_server
            .client()
            .get("http://localhost/posts")
            .perform()
            .unwrap();

        assert_eq!(response.status(), StatusCode::Ok);

        let body = response.read_body().unwrap();
        assert_eq!(&body[..], b"Hello World!");
    }
}
