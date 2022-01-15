use yew::{
    function_component, html, use_context, use_state, Children, ContextProvider, Html, Properties,
};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Count {
    count: usize,
}

impl Count {
    pub fn increment(&mut self) {
        self.count += 1;
    }

    pub fn current(&self) -> usize {
        self.count
    }
}

/// Main component
#[function_component(CounterProvider)]
pub fn counter_provider(props: &Props) -> Html {
    let ctx = use_state(|| Count { count: 0 });

    html! {
        <ContextProvider<Count> context={(*ctx).clone()}>
            // Every child here and their children will have access to this context.
            { for props.children.iter() }
        </ContextProvider<Count>>
    }
}

pub fn use_counter() -> Count {
    let ctx = use_context::<Count>().expect("no ctx found");

    ctx
}
