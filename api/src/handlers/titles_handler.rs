use diesel::prelude::*;
use gotham::http::response::create_response;
use gotham::state::State;
use hyper::{Response, StatusCode};
use mime;
use serde_json;

use models::db::establish_connection;
use models::post::posts::dsl::*;
use models::post::{posts, Post};
use models::title::{gen_titles};

pub fn index(state: State) -> (State, Response) {
    let connection = establish_connection();
    let results = posts
        .filter(published.eq(true))
        .order(posts::title)
        .load::<Post>(&connection)
        .expect("Error loading posts");
    let titles = gen_titles(results);
    let serialized = serde_json::to_string(&titles).unwrap();
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