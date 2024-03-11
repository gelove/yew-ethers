use crate::utils::dom::{get_value_from_event, get_value_from_input_event};
use web_sys::InputEvent;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub value: String,
    #[prop_or_default]
    pub placeholder: String,
    #[prop_or_default]
    pub extra_class: String,
    pub on_change: Callback<String>,
}

/// Controlled Text Input Component
#[function_component(TextInput)]
pub fn text_input(props: &Props) -> Html {
    let Props {
        value,
        placeholder,
        extra_class,
        on_change,
    } = props.clone();

    let oninput = {
        let on_change = on_change.clone();
        Callback::from(move |e: InputEvent| on_change.emit(get_value_from_input_event(e)))
    };

    let onchange = {
        let on_change = on_change.clone();
        Callback::from(move |e: Event| on_change.emit(get_value_from_event(e)))
    };

    html! {
        <input type="text" class={classes!("input", extra_class)} {placeholder} {value} {oninput} {onchange} />
    }
}
