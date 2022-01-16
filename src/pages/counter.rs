use crate::components::counter_button::CounterButton;
use crate::components::counter_display::CounterDisplay;
use yew::{function_component, html};

#[function_component(Counter)]
pub fn counter() -> Html {
    html! {
        <div class="flex flex-col justify-center items-center gap-4 w-full h-full ">
            <CounterDisplay />
            <CounterButton />
        </div>
    }
}
