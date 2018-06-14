use api::get_titles;
use failure::Error;
use models::Title;
use yew::format::Json;
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Response};

pub struct Context {
    pub web: FetchService,
}

impl AsMut<FetchService> for Context {
    fn as_mut(&mut self) -> &mut FetchService {
        &mut self.web
    }
}

#[derive(PartialEq, Clone)]
pub struct Props {
    pub onclick: Option<Callback<i32>>,
    pub updated: bool,
}

impl Default for Props {
    fn default() -> Self {
        Props {
            onclick: None,
            updated: false,
        }
    }
}

pub struct Menu {
    onclick: Option<Callback<i32>>,
    titles: Vec<Title>,
    ft: Option<FetchTask>,
    updated: bool,
}

pub enum Msg {
    OnClick(i32),
    FetchReady(Result<Vec<Title>, Error>),
    UpdateMenu,
    Ignore,
}

impl<CTX> Component<CTX> for Menu
where
    CTX: AsMut<FetchService> + 'static,
{
    type Msg = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, env: &mut Env<CTX, Self>) -> Self {
        let callback = env.send_back(|response: Response<Json<Result<Vec<Title>, Error>>>| {
            let (meta, Json(titles)) = response.into_parts();
            if meta.status.is_success() {
                Msg::FetchReady(titles)
            } else {
                Msg::Ignore // FIXME: Handle this error accordingly.
            }
        });
        let request = get_titles();
        let fetch_service: &mut FetchService = env.as_mut();
        let task = fetch_service.fetch(request, callback);
        Menu {
            onclick: props.onclick,
            titles: vec![],
            ft: Some(task),
            updated: props.updated,
        }
    }

    fn update(&mut self, msg: Self::Msg, env: &mut Env<CTX, Self>) -> ShouldRender {
        match msg {
            Msg::OnClick(post_id) => {
                if let Some(ref mut cb) = self.onclick {
                    cb.emit(post_id);
                }
            }
            Msg::FetchReady(data) => {
                self.updated = false;
                if let Ok(titles) = data {
                    self.titles = titles;
                }
            }
            Msg::UpdateMenu => {
                let callback =
                    env.send_back(|response: Response<Json<Result<Vec<Title>, Error>>>| {
                        let (meta, Json(titles)) = response.into_parts();
                        if meta.status.is_success() {
                            Msg::FetchReady(titles)
                        } else {
                            Msg::Ignore // FIXME: Handle this error accordingly.
                        }
                    });
                let request = get_titles();
                let fetch_service: &mut FetchService = env.as_mut();
                let task = fetch_service.fetch(request, callback);
                self.ft = Some(task);
            }
            Msg::Ignore => {
                self.updated = false;
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties, env: &mut Env<CTX, Self>) -> ShouldRender {
        self.onclick = props.onclick;
        if !self.updated && props.updated {
            self.updated = props.updated;
            self.update(Msg::UpdateMenu, env);
        } else {
            self.updated = props.updated;
        }
        true
    }
}

impl<CTX> Renderable<CTX, Menu> for Menu
where
    CTX: AsMut<FetchService> + 'static,
{
    fn view(&self) -> Html<CTX, Self> {
        html!{
            <nav>
              { for self.titles.clone().into_iter().map(|title| self.render_title(title, 0)) }
            </nav>
        }
    }
}

impl Menu {
    fn render_title<CTX>(&self, title: Title, level: i32) -> Html<CTX, Self>
    where
        CTX: AsMut<FetchService> + 'static,
    {
        let children_node: Html<CTX, Self> = html! { { for title.children.into_iter().map(|child| self.render_title(child, level + 1)) } };
        let sublink = match level {
            0 => "",
            1 => "sublink-1",
            2 => "sublink-2",
            _ => "sublink-3",
        };
        match title.post_id {
            Some(pid) => {
                html!{
                    <a class=("nav-item", sublink), onclick=move |_| Msg::OnClick(pid),>{ title.title }</a>
                    { children_node }
                }
            }
            None => {
                html!{
                    <span class=sublink,>{ title.title }</span>
                    { children_node }
                }
            }
        }
    }
}
