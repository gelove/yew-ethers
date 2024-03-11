use yew::{prelude::*, virtual_dom::AttrValue};

use crate::types::Token;
use crate::utils::dom;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub name: AttrValue,
    #[prop_or_default]
    pub options: Vec<Token>,
    #[prop_or_default]
    pub onchange: Callback<(AttrValue, String)>,
    // pub extra_class: String,
}

#[function_component(TokenSelect)]
pub fn token_select(props: &Props) -> Html {

    let _name = props.name.clone();
    let _onchange = props.onchange.clone();

    let onchange = {
        Callback::from(move |e: Event| {
            _onchange.emit((_name.clone(), dom::get_value_from_event(e)))
        })
    };

    html! {
        <select name={props.name.clone()} {onchange}>
            // {props.options.iter().map(|item| html!{<option selected={props.default.as_ref() == &item.address} value={item.address.clone()}>{format!("{} ({})", item.symbol, item.balance)}</option>}).collect::<Html>()}
            {props.options.iter().map(|item| html!{<option value={item.address.clone()}>{format!("{} ({})", item.symbol, item.balance)}</option>}).collect::<Html>()}
        </select>
    }
}
