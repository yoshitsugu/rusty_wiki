use models::NewPost;
use yew::prelude::*;
use yew::services::dialog::DialogService;

pub struct Context {
    pub dialog: DialogService,
}

impl AsMut<DialogService> for Context {
    fn as_mut(&mut self) -> &mut DialogService {
        &mut self.dialog
    }
}

#[derive(PartialEq, Clone)]
pub struct Props {
    pub post: NewPost,
    pub oncancel: Option<Callback<()>>,
    pub onsubmit: Option<Callback<NewPost>>,
    pub ondelete: Option<Callback<()>>,
}

impl Default for Props {
    fn default() -> Self {
        Props {
            post: NewPost {
                title: "".to_string(),
                body: "".to_string(),
            },
            oncancel: None,
            onsubmit: None,
            ondelete: None,
        }
    }
}

pub struct PostForm {
    post: NewPost,
    oncancel: Option<Callback<()>>,
    onsubmit: Option<Callback<NewPost>>,
    ondelete: Option<Callback<()>>,
}

pub enum Msg {
    UpdateTitle(String),
    UpdateBody(String),
    OnCancel,
    OnSubmit,
    OnDelete,
}

impl<CTX> Component<CTX> for PostForm
where
    CTX: AsMut<DialogService> + 'static,
{
    type Msg = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: &mut Env<CTX, Self>) -> Self {
        PostForm {
            post: props.post,
            oncancel: props.oncancel,
            onsubmit: props.onsubmit,
            ondelete: props.ondelete,
        }
    }

    fn update(&mut self, msg: Self::Msg, env: &mut Env<CTX, Self>) -> ShouldRender {
        match msg {
            Msg::UpdateTitle(title) => {
                self.post = NewPost {
                    title: title,
                    ..self.post.clone()
                };
            }
            Msg::UpdateBody(body) => {
                self.post = NewPost {
                    body: body,
                    ..self.post.clone()
                };
            }
            Msg::OnCancel => {
                if let Some(ref mut cb) = self.oncancel {
                    cb.emit(());
                }
            }
            Msg::OnSubmit => {
                if let Some(ref mut cb) = self.onsubmit {
                    cb.emit(self.post.clone());
                }
            }
            Msg::OnDelete => {
                if let Some(ref mut cb) = self.ondelete {
                    let dialog: &mut DialogService = env.as_mut();
                    if dialog.confirm("Are you sure?") {
                        cb.emit(());
                    }
                }
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties, _: &mut Env<CTX, Self>) -> ShouldRender {
        self.post = props.post;
        self.oncancel = props.oncancel;
        self.onsubmit = props.onsubmit;
        self.ondelete = props.ondelete;
        true
    }
}

impl<CTX> Renderable<CTX, PostForm> for PostForm
where
    CTX: AsMut<DialogService> + 'static,
{
    fn view(&self) -> Html<CTX, Self> {
        html! {
          <div class=("card", "fluid"),>
            <form>
              <div class="row",>
                <div class="col-sm-12",>
                  <input id="title",
                         type="text",
                         oninput=|e: InputData| Msg::UpdateTitle(e.value),
                         style="width: 100%",
                         value=&self.post.title,
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
                    { &self.post.body }
                  </textarea>
                </div>
              </div>
            </form>
            <div class="row",>
              <div class=("col-sm-12", "buttons"),>
                <button onclick=|_| Msg::OnCancel,>{ "cancel" }</button>
                <button class="primary", onclick=|_| Msg::OnSubmit,>{ "submit" }</button>
              </div>
            </div>
            { self.delete_button() }
          </div>
        }
    }
}

impl PostForm {
    pub fn delete_button<CTX>(&self) -> Html<CTX, Self>
    where
        CTX: AsMut<DialogService> + 'static,
    {
        if let Some(_) = self.ondelete {
            html!{
              <div class="row",>
                <div class=("col-sm-12", "buttons"),>
                  <button class="secondary", onclick=|_| Msg::OnDelete,>{ "delete" }</button>
                </div>
              </div>
            }
        } else {
            html!{}
        }
    }
}
