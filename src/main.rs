use yew::prelude::*;
use yew_app::components::button::Button;
use yew_app::components::counter_button::CounterButton;
use yew_app::components::counter_display::CounterDisplay;
use yew_app::contexts::counter_context::CounterProvider;

enum Msg {
    AddOne,
}

struct Model {
    value: i64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { value: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();

        html! {
            <CounterProvider>
                <div>
                    <p>{ self.value }</p>
                    <Button title={String::from("ボタン1")} onclick={link.callback(|_: MouseEvent| Msg::AddOne)}/>
                    <CounterDisplay />
                    <CounterButton />
                </div>
            </CounterProvider>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Model>();
}
