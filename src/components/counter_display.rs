use crate::contexts::counter_context;
use yew::{function_component, html};

#[function_component(CounterDisplay)]
pub fn counter_display() -> Html {
    let counter = counter_context::use_counter();
    html! {
        <div>
            { format!("Current count is : {}", counter.current()) }
        </div>
    }
}
