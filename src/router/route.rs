use yew_router::prelude::Routable;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,

    #[at("/counter")]
    Counter,

    #[not_found]
    #[at("/404")]
    NotFound,
}
