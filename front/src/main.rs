#![recursion_limit = "128"]

#[macro_use]
extern crate yew;
extern crate failure;
#[macro_use]
extern crate stdweb;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
use failure::Error;
use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::console::ConsoleService;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

use stdweb::unstable::TryFrom;
use stdweb::web::Node;

use yew::virtual_dom::VNode;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub body_html: String,
    pub published: bool,
}

struct Context {
    console: ConsoleService,
    web: FetchService,
}

impl AsMut<ConsoleService> for Context {
    fn as_mut(&mut self) -> &mut ConsoleService {
        &mut self.console
    }
}

impl AsMut<FetchService> for Context {
    fn as_mut(&mut self) -> &mut FetchService {
        &mut self.web
    }
}

struct Model {
    fetching: bool,
    ft: Option<FetchTask>,
    post: Option<Post>,
    editing: bool,
}

enum Msg {
    FetchData(i32),
    FetchReady(Result<Post, Error>),
    PutData,
    SetEditing(bool),
    UpdateTitle(String),
    UpdateBody(String),
    Ignore,
}

impl<CTX> Component<CTX> for Model
where
    CTX: AsMut<FetchService> + AsMut<ConsoleService> + 'static,
{
    type Msg = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<CTX, Self>) -> Self {
        Model {
            fetching: false,
            post: None,
            ft: None,
            editing: false,
        }
    }

    fn update(&mut self, msg: Self::Msg, env: &mut Env<CTX, Self>) -> ShouldRender {
        match msg {
            Msg::FetchData(num) => {
                self.fetching = true;
                self.editing = false;
                let callback = env.send_back(|response: Response<Json<Result<Post, Error>>>| {
                    let (meta, Json(data)) = response.into_parts();
                    println!("META: {:?}, {:?}", meta, data);
                    if meta.status.is_success() {
                        Msg::FetchReady(data)
                    } else {
                        Msg::Ignore // FIXME: Handle this error accordingly.
                    }
                });
                let request = Request::get(format!("http://localhost:8083/posts/{}", num))
                    .body(Nothing)
                    .unwrap();
                let fetch_service: &mut FetchService = env.as_mut();
                let task = fetch_service.fetch(request, callback);
                self.ft = Some(task);
            }
            Msg::FetchReady(response) => {
                self.fetching = false;
                self.editing = false;
                self.post = response.ok();
            }
            Msg::PutData => {
                if let Some(post_data) = self.post.clone() {
                    self.fetching = true;
                    let callback =
                        env.send_back(|response: Response<Json<Result<Post, Error>>>| {
                            let (meta, Json(data)) = response.into_parts();
                            if meta.status.is_success() {
                                Msg::FetchReady(data)
                            } else {
                                Msg::Ignore // FIXME: Handle this error accordingly.
                            }
                        });
                    let request =
                        Request::put(format!("http://localhost:8083/posts/{}", post_data.id))
                            .body(serde_json::to_string(&post_data).unwrap())
                            .unwrap();
                    let fetch_service: &mut FetchService = env.as_mut();
                    let task = fetch_service.fetch(request, callback);
                    self.ft = Some(task);
                }
            }
            Msg::SetEditing(editing) => {
                self.editing = editing;
            }
            Msg::UpdateTitle(title) => {
                if let Some(post_data) = self.post.clone() {
                    self.post = Some(Post {
                        title: title,
                        ..post_data
                    });
                }
            }
            Msg::UpdateBody(body) => {
                if let Some(post_data) = self.post.clone() {
                    self.post = Some(Post {
                        body: body,
                        ..post_data
                    });
                }
            }
            Msg::Ignore => {
                let console_service: &mut ConsoleService = env.as_mut();
                self.post = None;
                console_service.log("Ignore");
            }
        }
        true
    }
}

impl<CTX> Renderable<CTX, Model> for Model
where
    CTX: AsMut<FetchService> + AsMut<ConsoleService> + 'static,
{
    fn view(&self) -> Html<CTX, Self> {
        html! {
            <div>
              <header>
                <div class="container",>
                  <div class="row",>
                    <a href="#", class=("logo", "col-sm-8"),>{ "RustyWiki" }</a>
                    <a href="#", class=("button", "col-sm-4", "user_menu"),>
                      <span class="icon-user",></span>
                      { "User" }
                    </a>
                  </div>
                </div>
              </header>
              <div class="container",>
                <div class="row",>
                  <div class="col-sm-4",>
                    <nav>
                      <a class="nav-item", onclick=|_| Msg::FetchData(2),>{ "post 2" }</a>
                      <a class="nav-item", onclick=|_| Msg::FetchData(3),>{ "post 3" }</a>
                      <a class="nav-item", onclick=|_| Msg::FetchData(4),>{ "post 4" }</a>
                    </nav>
                  </div>
                  <div class=("main", "col-sm-8"),>
                    { self.show_post_html() }
                  </div>
                </div>
              </div>
            </div>
        }
    }
}

impl Model {
    fn show_post_html<CTX>(&self) -> Html<CTX, Model>
    where
        CTX: AsMut<FetchService> + AsMut<ConsoleService> + 'static,
    {
        if let Some(ref value) = self.post {
            if self.editing {
                html! {
                  <div class=("card", "fluid"),>
                    <form>
                      <div class="row",>
                        <div class="col-sm-12",>
                          <input id="title",
                                 type="text",
                                 oninput=|e: InputData| Msg::UpdateTitle(e.value),
                                 style="width: 100%",
                                 value=&value.title,
                                 placeholder="Title", />
                        </div>
                      </div>
                      <div class="row",>
                        <div class="col-sm-12",>
                          <textarea id="body",
                                    rows=20,
                                    type="text",
                                    style="width: 100%",
                                    oninput=|e: InputData| Msg::UpdateBody(e.value),
                          >
                            { &value.body }
                          </textarea>
                        </div>
                      </div>
                    </form>
                    <div class="row",>
                      <div class=("col-sm-12", "buttons"),>
                        <button onclick=|_| Msg::SetEditing(false),>{ "cancel" }</button>
                        <button class="primary", onclick=|_| Msg::PutData,>{ "submit" }</button>
                      </div>
                    </div>
                  </div>
                }
            } else {
                html! {
                  <div class=("card", "fluid"),>
                    <div class=("section", "row", "post_title_wrapper"),>
                      <div class=("post_title", "col-sm-9"),>{ &value.title }</div>
                      <div class=("post_edit", "col-sm-3"),>
                        <a onclick=|_| Msg::SetEditing(true),><span class="icon-edit",></span></a>
                      </div>
                    </div>
                    <div class="section",>
                      { self.post_body_html(value) }
                    </div>
                  </div>
                }
            }
        } else {
            html! {
              <div class=("card", "fluid"),>
                <div class="section",>{ "Data hasn't fetched yet." }</div>
              </div>
            }
        }
    }

    fn post_body_html<CTX>(&self, post: &Post) -> Html<CTX, Model>
    where
        CTX: AsMut<FetchService> + AsMut<ConsoleService> + 'static,
    {
        let post_div = js! {
            var div = document.createElement("div");
            div.classList.add("section");
            div.innerHTML = @{post.body_html.to_string()};
            return div;
        };
        let node = Node::try_from(post_div).expect("convert body_html");
        VNode::VRef(node)
    }
}

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
