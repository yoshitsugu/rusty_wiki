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

pub mod api;
pub mod components;
pub mod models;

use failure::Error;
use models::{NewPost, Post};
use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::console::ConsoleService;
use yew::services::dialog::DialogService;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

use stdweb::unstable::TryFrom;
use stdweb::web::Node;

use yew::virtual_dom::VNode;

use components::menu::Menu;
use components::post_form::PostForm;

const API_HOSTNAME: &'static str = env!("API_HOSTNAME");

pub struct Context {
    pub console: ConsoleService,
    pub web: FetchService,
    pub dialog: DialogService,
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

impl AsMut<DialogService> for Context {
    fn as_mut(&mut self) -> &mut DialogService {
        &mut self.dialog
    }
}

pub struct Model {
    fetching: bool,
    ft: Option<FetchTask>,
    post: Option<Post>,
    editing: bool,
    menu_updated: bool,
}

pub enum Msg {
    FetchData(i32),
    FetchReady(Result<Post, Error>),
    FetchBlank,
    PutData(NewPost),
    SetEditing(bool),
    NewPost,
    PostData(NewPost),
    DeleteData,
    Ignore,
}

impl<CTX> Component<CTX> for Model
where
    CTX: AsMut<FetchService> + AsMut<DialogService> + AsMut<ConsoleService> + 'static,
{
    type Msg = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<CTX, Self>) -> Self {
        Model {
            fetching: false,
            post: None,
            ft: None,
            editing: false,
            menu_updated: false,
        }
    }

    fn update(&mut self, msg: Self::Msg, env: &mut Env<CTX, Self>) -> ShouldRender {
        match msg {
            Msg::FetchData(num) => {
                self.fetching = true;
                self.editing = false;
                let callback = env.send_back(|response: Response<Json<Result<Post, Error>>>| {
                    let (meta, Json(data)) = response.into_parts();
                    if meta.status.is_success() {
                        Msg::FetchReady(data)
                    } else {
                        Msg::Ignore // FIXME: Handle this error accordingly.
                    }
                });
                let request = Request::get(format!("http://{}/posts/{}", API_HOSTNAME, num))
                    .body(Nothing)
                    .unwrap();
                let fetch_service: &mut FetchService = env.as_mut();
                let task = fetch_service.fetch(request, callback);
                self.ft = Some(task);
            }
            Msg::FetchReady(response) => {
                self.fetching = false;
                self.editing = false;
                self.menu_updated = true;
                self.post = response.ok();
            }
            Msg::FetchBlank => {
                self.fetching = false;
                self.editing = false;
                self.menu_updated = true;
                self.post = None;
            }
            Msg::SetEditing(editing) => {
                self.editing = editing;
            }
            Msg::PutData(new_post) => {
                if let Some(ref post) = self.post {
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
                        Request::put(format!("http://{}/posts/{}", API_HOSTNAME, post.id))
                            .body(serde_json::to_string(&new_post).unwrap())
                            .unwrap();
                    let fetch_service: &mut FetchService = env.as_mut();
                    let task = fetch_service.fetch(request, callback);
                    self.ft = Some(task);
                }
            }
            Msg::NewPost => {
                self.post = None;
                self.editing = true;
            }
            Msg::PostData(new_post) => {
                self.fetching = true;
                let callback = env.send_back(|response: Response<Json<Result<Post, Error>>>| {
                    let (meta, Json(data)) = response.into_parts();
                    if meta.status.is_success() {
                        Msg::FetchReady(data)
                    } else {
                        Msg::Ignore // FIXME: Handle this error accordingly.
                    }
                });
                let request = Request::post(format!("http://{}/posts", API_HOSTNAME))
                    .body(serde_json::to_string(&new_post).unwrap())
                    .unwrap();
                let fetch_service: &mut FetchService = env.as_mut();
                let task = fetch_service.fetch(request, callback);
                self.ft = Some(task);
            }
            Msg::DeleteData => {
                if let Some(ref post) = self.post {
                    self.fetching = true;
                    let callback =
                        env.send_back(|response: Response<Json<Result<String, Error>>>| {
                            let (meta, _) = response.into_parts();
                            if meta.status.is_success() {
                                Msg::FetchBlank
                            } else {
                                Msg::Ignore // FIXME: Handle this error accordingly.
                            }
                        });
                    let request =
                        Request::delete(format!("http://{}/posts/{}", API_HOSTNAME, post.id))
                            .body(Nothing)
                            .unwrap();
                    let fetch_service: &mut FetchService = env.as_mut();
                    let task = fetch_service.fetch(request, callback);
                    self.ft = Some(task);
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
    CTX: AsMut<FetchService> + AsMut<DialogService> + AsMut<ConsoleService> + 'static,
{
    fn view(&self) -> Html<CTX, Self> {
        html! {
            <div>
              <header>
                <div class="container",>
                  <div class="row",>
                    <a href="#", class=("logo", "col-sm-8"),>{ "RustyWiki" }</a>
                    <a class=("button", "col-sm-4", "user_menu"), onclick=|_| Msg::NewPost,>
                      <span class="icon-plus-square",></span>
                      { "New Page" }
                    </a>
                  </div>
                </div>
              </header>
              <div class="container",>
                <div class="row",>
                  <div class="col-sm-4",>
                    <Menu: onclick=|post_id| Msg::FetchData(post_id), updated=self.menu_updated,/>
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
        CTX: AsMut<FetchService> + AsMut<DialogService> + AsMut<ConsoleService> + 'static,
    {
        if let Some(ref value) = self.post {
            if self.editing {
                html! {
                  <PostForm: post=value.to_new_post(),
                  oncancel=|_| Msg::SetEditing(false),
                  onsubmit=|new_post| Msg::PutData(new_post),
                  ondelete=|_| Msg::DeleteData,
                  />
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
            if self.editing {
                html! {
                  <PostForm: post=NewPost::new(),
                  oncancel=|_| Msg::SetEditing(false),
                  onsubmit=|new_post| Msg::PostData(new_post),
                  ondelete=None,
                  />
                }
            } else {
                html! {
                  <div class=("card", "fluid"),>
                    <div class="section", style="min-height: 300px;", >
                      <h2>{ "RustyWiki HOME" }</h2>
                    </div>
                  </div>
                }
            }
        }
    }

    fn post_body_html<CTX>(&self, post: &Post) -> Html<CTX, Model>
    where
        CTX: AsMut<FetchService> + AsMut<DialogService> + AsMut<ConsoleService> + 'static,
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
