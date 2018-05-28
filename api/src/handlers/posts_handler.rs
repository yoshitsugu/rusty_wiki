use super::super::models::db::establish_connection;
use super::super::models::post::Post;
use diesel::prelude::*;
use gotham::http::response::create_response;
use gotham::state::State;
use hyper::{Response, StatusCode};
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
