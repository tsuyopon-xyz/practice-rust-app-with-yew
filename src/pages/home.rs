use yew::{function_component, html};

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class="bg-white min-h-full px-4 py-16 sm:px-6 sm:py-24 md:grid md:place-items-center lg:px-8">
            <div class="max-w-max mx-auto">
                <main class="sm:flex">
                    <p class="text-4xl font-extrabold text-indigo-600 sm:text-5xl">{ "This is Home!!!" }</p>
                </main>
            </div>
        </div>
    }
}
