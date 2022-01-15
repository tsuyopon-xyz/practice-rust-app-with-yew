use crate::contexts::counter_context;
use yew::{function_component, html, use_effect_with_deps, Callback};

#[function_component(CounterButton)]
pub fn counter_button() -> Html {
    let counter = counter_context::use_counter();
    let onclick = Callback::from(move |_| {
        log::info!("TODO: incremnt in Callback {:#?}", counter);
        todo!();
        // counter.increment();
    });

    html! {
        <button onclick={onclick} type="button" class="inline-flex items-center px-6 py-3 border border-transparent text-base font-medium rounded-md shadow-sm text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500">
            { "カウンターボタン" }
        </button>
    }
}
