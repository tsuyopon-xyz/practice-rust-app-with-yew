use std::rc::Rc;
use yew::{
    function_component, html, use_context, use_reducer, Children, ContextProvider, Properties,
    Reducible, UseReducerHandle,
};

#[derive(Properties, Clone, PartialEq, Debug)]
pub struct Props {
    pub children: Children,
}

pub enum CounterAction {
    Increment,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Counter {
    pub value: usize,
}

impl Default for Counter {
    fn default() -> Self {
        Self { value: 0 }
    }
}

impl Reducible for Counter {
    /// Reducer Action Type
    type Action = CounterAction;

    /// Reducer Function
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let next_ctr = match action {
            CounterAction::Increment => self.value + 1,
        };

        Self { value: next_ctr }.into()
    }
}

#[function_component(CounterProvider)]
pub fn counter_provider(props: &Props) -> Html {
    let counter = use_reducer(Counter::default);

    html! {
        <ContextProvider<UseReducerHandle<Counter>> context={counter}>
            // Every child here and their children will have access to this context.
            { for props.children.iter() }
        </ContextProvider<UseReducerHandle<Counter>>>
    }
}

pub fn use_counter() -> UseReducerHandle<Counter> {
    let ctx = use_context::<UseReducerHandle<Counter>>().expect("no ctx found");

    ctx
}
