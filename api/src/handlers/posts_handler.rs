use super::super::models::db::establish_connection;
use super::super::models::post::{Post, NewPost};
use diesel::prelude::*;
use diesel;
use gotham::handler::{HandlerFuture, IntoHandlerError};
use gotham::http::response::create_response;
use gotham::state::{FromState, State};
use hyper::{Body, Response, StatusCode};
use futures::{future, Future, Stream};
use mime;
use serde_json;

pub fn show(state: State) -> (State, Response) {
    use super::super::models::post::posts::dsl::*;

    let connection = establish_connection();
    let results = posts
        .filter(published.eq(true))
        .limit(1)
        .load::<Post>(&connection)
        .expect("Error loading posts");
    let result = results.get(0).unwrap();
    let serialized = serde_json::to_string(&result).unwrap();
    let res = create_response(
        &state,
        StatusCode::Ok,
        Some((
            String::from(serialized).into_bytes(),
            mime::APPLICATION_JSON,
        )),
    );

    (state, res)
}

pub fn post(mut state: State) -> Box<HandlerFuture> {
    use super::super::models::post::posts;
    let f = Body::take_from(&mut state)
        .concat2()
        .then(|full_body| match full_body {
            Ok(valid_body) => {
                let body_content = String::from_utf8(valid_body.to_vec()).unwrap();
                let mut post: NewPost = serde_json::from_str(&body_content).unwrap();
                match post.generate_body_html() {
                    Ok(_html) => {
                      if post.title.len() > 0 {
                        let connection = establish_connection();
                        let saved_post: Post = diesel::insert_into(posts::table)
                          .values(&post)
                          .get_result(&connection)
                          .expect("Error inserting post");
                        let serialized = serde_json::to_string(&saved_post).unwrap();
                        let res = create_response(
                            &state,
                            StatusCode::Ok,
                            Some((
                                String::from(serialized).into_bytes(),
                                mime::APPLICATION_JSON
                                )));
                        future::ok((state, res))
                      } else {
                        let res = create_response(&state, StatusCode::Ok, None);
                        future::ok((state, res))
                      }
                    },
                    Err(e) => return future::err((state, e.into_handler_error()))
                }
            }
            Err(e) => return future::err((state, e.into_handler_error())),
        });

    Box::new(f)
}
