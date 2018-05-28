#[macro_use]
extern crate yew;
use yew::prelude::*;
use yew::services::console::ConsoleService;


struct Context {
    console: ConsoleService,
}

impl AsMut<ConsoleService> for Context {
    fn as_mut(&mut self) -> &mut ConsoleService {
        &mut self.console
    }
}

struct Model {}

enum Msg {
    DoIt,
}

impl<CTX> Component<CTX> for Model
where
    CTX: AsMut<ConsoleService>,
{
    type Msg = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<CTX, Self>) -> Self {
        Model {}
    }

    fn update(&mut self, msg: Self::Msg, env: &mut Env<CTX, Self>) -> ShouldRender {
        match msg {
            Msg::DoIt => {
                // Update your model on events
                env.as_mut().log("do it!!");
            }
        }
        true
    }
}

impl<CTX> Renderable<CTX, Model> for Model
where
    CTX: AsMut<ConsoleService> + 'static,
{
    fn view(&self) -> Html<CTX, Self> {
        html! {
            <div id="container",>
              <div class="menu",>
                <div class="title",>{ "rusty wiki" }</div>
                <ul class="liinks",>
                  <li class="link",>{ "このwikiについて" }</li>
                  <li class="link",>{ "メインコンテンツA" }</li>
                  <li class="link",>{ "てすと" }</li>
                </ul>
              </div>
              <div class="main",>
                <div>
                  <h3>{ "#hogehoge" }</h3>
                  <p> 
                    { "piyopiyoaaaaaaaaaaaaaaああああああああああああああああ" }
                    <a href="#",>{ "あかじゃいあ" }</a>
                    { "あala木曾路はすべて山の中である。あるところは岨づたいに行く崖の道であり、あるところは数十間の深さに臨む木曾川の岸であり、あるところは山の尾をめぐる谷の入り口である。一筋の街道はこの深い森林地帯を貫いていた。東ざかいの桜沢から、西の十曲峠まで、木曾十一宿はこの街道に添うて、二十二里余にわたる長い谿谷の間に散在していた。道路の位置も幾たびか改まったもので、古道はいつのまにか深い山間に埋もれた。名高い桟も、蔦のかずらを頼みにしたような危い場処ではなくなって、徳川時代の末にはすでに渡ることのできる橋であった。新規に新規にとできた道はだんだん谷の下の方の位置へと降って来た。道の狭いところには、木を伐って並べ、藤づるでからめ、それで街道の狭いのを補った。長い間にこの木曾路に起こって来た変化は、いくらかずつでも嶮岨な山坂の多いところを歩きよくした。そのかわり、大雨ごとにやって来る河水の氾濫が旅行を困難にする" }
                  </p>
                  <br />
                  <h4>{ "hogepiyo" }</h4>
                </div>// Render your model here
              </div>
            </div>
        }
    }
}

fn main() {
    yew::initialize();
    let context = Context {
        console: ConsoleService::new(),
    };
    let app: App<_, Model> = App::new(context);
    app.mount_to_body();
    yew::run_loop();
}
