use yew::{function_component, html, Html};
use yew_app::components::header::Header;
use yew_app::contexts::counter_context::CounterProvider;
use yew_app::pages::{Counter, Home, NotFound};
use yew_app::router::route::Route;
use yew_router::prelude::{BrowserRouter, Switch};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Header />
            <CounterProvider>
                <Switch<Route> render={Switch::render(switch)} />
            </CounterProvider>
        </BrowserRouter>
    }
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Counter => html! { <Counter /> },
        Route::NotFound => html! { <NotFound /> },
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
