extern crate yew;
extern crate failure;
extern crate stdweb;
extern crate serde;
extern crate serde_json;
extern crate rusty_wiki_front;

use yew::prelude::*;
use yew::services::console::ConsoleService;
use yew::services::fetch::FetchService;
use rusty_wiki_front::{Context, Model};

fn main() {
    yew::initialize();
    let context = Context {
        console: ConsoleService::new(),
        web: FetchService::new(),
    };
    let app: App<Context, Model> = App::new(context);
    app.mount_to_body();
    yew::run_loop();
}
