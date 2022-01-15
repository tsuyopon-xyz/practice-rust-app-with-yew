use yew::prelude::*;
use yew_app::components::button::Button;

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
        let onclick = |_: MouseEvent| Msg::AddOne;

        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        let callback = link.callback(onclick);
        html! {
            <div>
                // <button onclick={link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
                // <button onclick={callback}>{ "+1" }</button>
                <p>{ self.value }</p>
                <Button title={String::from("ボタン")} onclick={callback}/>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
