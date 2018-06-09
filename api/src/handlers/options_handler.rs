use gotham::http::response::create_response;
use gotham::state::State;
use hyper::{Response, StatusCode};

pub fn ok(state: State) -> (State, Response) {
    let res = create_response(&state, StatusCode::Ok, None);

    (state, res)
}
