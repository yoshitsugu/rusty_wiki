extern crate gotham;
extern crate hyper;
extern crate rusty_wiki;

use gotham::router::Router;
use gotham::router::builder::*;

use rusty_wiki::handlers::*;

fn router() -> Router {
    build_simple_router(|route| {
        route.get("/posts").to(posts_handler::index);
        route.get("/posts/:id").with_path_extractor::<posts_handler::PostPathExtractor>().to(posts_handler::show);
        route.post("/posts").to(posts_handler::post);
        route.options("/posts/:id").to(options_handler::ok);
        route.options("/posts").to(options_handler::ok);
        route.put("/posts/:id").with_path_extractor::<posts_handler::PostPathExtractor>().to(posts_handler::update);
        route.delete("/posts/:id").with_path_extractor::<posts_handler::PostPathExtractor>().to(posts_handler::delete);
    })
}

pub fn main() {
    let addr = "0.0.0.0:7878";
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
