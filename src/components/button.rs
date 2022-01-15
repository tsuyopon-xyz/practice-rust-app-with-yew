use yew::{function_component, html, Callback, MouseEvent, Properties};

#[derive(Properties, PartialEq)]
pub struct RenderedAtProps {
    pub title: String,
    pub onclick: Option<Callback<MouseEvent>>,
}

#[function_component(Button)]
pub fn button(props: &RenderedAtProps) -> Html {
    if let Some(callback) = &props.onclick {
        html! {
            <button onclick={callback} type="button" class="inline-flex items-center px-6 py-3 border border-transparent text-base font-medium rounded-md shadow-sm text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500">
                { &props.title }
            </button>
        }
    } else {
        html! {
            <button type="button" class="inline-flex items-center px-6 py-3 border border-transparent text-base font-medium rounded-md shadow-sm text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500">
                { &props.title }
            </button>
        }
    }
}
