use super::super::models::db::establish_connection;
use super::super::models::post::{NewPost, Post};
use diesel;
use diesel::prelude::*;
use futures::{future, Future, Stream};
use gotham::handler::{HandlerFuture, IntoHandlerError};
use gotham::http::response::create_response;
use gotham::state::{FromState, State};
use hyper::{Body, Response, StatusCode};
use mime;
use serde_json;

use super::super::models::post::posts;
use super::super::models::post::posts::dsl::*;

#[derive(Debug, Deserialize, StateData, StaticResponseExtender)]
pub struct PostPathExtractor {
    id: i32,
}

pub fn index(state: State) -> (State, Response) {
    let connection = establish_connection();
    let results = posts
        .filter(published.eq(true))
        .limit(20)
        .load::<Post>(&connection)
        .expect("Error loading posts");
    let serialized = serde_json::to_string(&results).unwrap();
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

pub fn show(state: State) -> (State, Response) {
    let res = {
        let path_params = PostPathExtractor::borrow_from(&state);
        let connection = establish_connection();
        let result = posts
            .filter(published.eq(true))
            .find(path_params.id)
            .first::<Post>(&connection);
        match result {
            Ok(post) => {
                let serialized = serde_json::to_string(&post).unwrap();
                create_response(
                    &state,
                    StatusCode::Ok,
                    Some((
                        String::from(serialized).into_bytes(),
                        mime::APPLICATION_JSON,
                    )),
                )
            }
            Err(err) => {
                println!("{:?}", err);
                create_response(
                    &state,
                    StatusCode::NotFound,
                    Some((
                        String::from("{error: not_found}").into_bytes(),
                        mime::APPLICATION_JSON,
                    )),
                )
            }
        }
    };

    (state, res)
}

pub fn post(mut state: State) -> Box<HandlerFuture> {
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
                                    mime::APPLICATION_JSON,
                                )),
                            );
                            future::ok((state, res))
                        } else {
                            let res = create_response(&state, StatusCode::Ok, None);
                            future::ok((state, res))
                        }
                    }
                    Err(e) => return future::err((state, e.into_handler_error())),
                }
            }
            Err(e) => return future::err((state, e.into_handler_error())),
        });

    Box::new(f)
}

pub fn update(mut state: State) -> Box<HandlerFuture> {
    let f = Body::take_from(&mut state)
        .concat2()
        .then(|full_body| match full_body {
            Ok(valid_body) => {
                let connection = establish_connection();
                let body_content = String::from_utf8(valid_body.to_vec()).unwrap();
                let mut post: NewPost = serde_json::from_str(&body_content).unwrap();
                match post.generate_body_html() {
                    Ok(_html) => {
                        let result: Result<Post, _> = {
                            let path_params = PostPathExtractor::borrow_from(&state);
                            diesel::update(posts.find(path_params.id))
                                .set(&post)
                                .get_result(&connection)
                        };
                        match result {
                            Ok(updated_post) => {
                                let serialized = serde_json::to_string(&updated_post).unwrap();
                                let res = create_response(
                                    &state,
                                    StatusCode::Ok,
                                    Some((
                                        String::from(serialized).into_bytes(),
                                        mime::APPLICATION_JSON,
                                    )),
                                );
                                future::ok((state, res))
                            }
                            Err(e) => return future::err((state, e.into_handler_error())),
                        }
                    }
                    Err(e) => return future::err((state, e.into_handler_error())),
                }
            }
            Err(e) => return future::err((state, e.into_handler_error())),
        });
    Box::new(f)
}

pub fn delete(state: State) -> (State, Response) {
    let connection = establish_connection();
    let response = {
        let path_params = PostPathExtractor::borrow_from(&state);
        diesel::delete(posts.find(path_params.id))
            .execute(&connection)
            .unwrap();
        create_response(&state, StatusCode::Ok, None)
    };
    (state, response)
}
