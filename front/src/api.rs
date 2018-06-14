use yew::format::Nothing;
use yew::services::fetch::Request;

const API_HOSTNAME: &'static str = env!("API_HOSTNAME");

pub fn get_titles() -> Request<Nothing> {
    Request::get(format!("http://{}/titles", API_HOSTNAME)).body(Nothing).expect("Error in getting titles")
}
