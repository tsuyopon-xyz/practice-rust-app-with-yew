use crate::router::route::Route;
use yew::{function_component, html};
use yew_router::prelude::Link;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header class="bg-indigo-600 fixed w-full">
            <nav class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8" aria-label="Top">
                <div class="w-full py-6 flex items-center justify-between border-b border-indigo-500 lg:border-none">
                    <div class="flex items-center">
                        <div class="space-x-8">
                            <Link<Route> to={Route::Home} classes="text-base font-medium text-white hover:text-indigo-50">{ "Home" }</Link<Route>>
                            <Link<Route> to={Route::Counter} classes="ml-10 text-base font-medium text-white hover:text-indigo-50">{ "Counter" }</Link<Route>>
                            <Link<Route> to={Route::NotFound} classes="ml-10 text-base font-medium text-white hover:text-indigo-50">{"Not Found"}</Link<Route>>
                        </div>
                    </div>
                </div>
            </nav>
        </header>
    }
}
