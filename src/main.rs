use yew::{function_component, html};
use yew_app::components::counter_button::CounterButton;
use yew_app::components::counter_display::CounterDisplay;
use yew_app::contexts::counter_context::CounterProvider;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <CounterProvider>
            <div class="flex flex-col justify-center items-center gap-4 w-full h-full">
                <CounterDisplay />
                <CounterButton />
            </div>
        </CounterProvider>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
