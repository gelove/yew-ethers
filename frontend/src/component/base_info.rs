use gloo::events::EventListener;
use yew::{prelude::*, virtual_dom::AttrValue};

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub name: AttrValue,
    pub age: u8,
    pub icon: AttrValue,
}

#[function_component(BaseInfo)]
pub fn base_info(props: &Props) -> Html {
    use_effect_with_deps(
        |name| {
            let body = gloo::utils::body();
            let name = name.clone();

            let listener = EventListener::new(&body, "click", move |_event| {
                log::debug!("name: {}", name);
            });

            || {
                // Gloo EventListeners are deregistered when it's dropped.
                let _listener = listener;
            }
        },
        props.name.clone(),
    );

    html! {
        <div>
            // example
        </div>
    }
}
