use models::NewPost;
use yew::prelude::*;

#[derive(PartialEq, Clone)]
pub struct Props {
    pub post: NewPost,
    pub oncancel: Option<Callback<()>>,
    pub onsubmit: Option<Callback<NewPost>>,
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
        }
    }
}

pub struct PostForm {
    post: NewPost,
    oncancel: Option<Callback<()>>,
    onsubmit: Option<Callback<NewPost>>,
}

pub enum Msg {
    UpdateTitle(String),
    UpdateBody(String),
    OnCancel,
    OnSubmit,
}

impl<CTX: 'static> Component<CTX> for PostForm {
    type Msg = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: &mut Env<CTX, Self>) -> Self {
        PostForm {
            post: props.post,
            oncancel: props.oncancel,
            onsubmit: props.onsubmit,
        }
    }

    fn update(&mut self, msg: Self::Msg, _: &mut Env<CTX, Self>) -> ShouldRender {
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
        }
        true
    }

    fn change(&mut self, props: Self::Properties, _: &mut Env<CTX, Self>) -> ShouldRender {
        self.post = props.post;
        true
    }
}

impl<CTX: 'static> Renderable<CTX, PostForm> for PostForm {
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
          </div>
        }
    }
}
