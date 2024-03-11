use crate::utils::dom::get_value_from_event;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub name: String,
    #[prop_or("text".to_string())]
    pub kind: String,
    #[prop_or("input".to_string())]
    pub title: String,
    #[prop_or_default]
    pub placeholder: String,
    #[prop_or_default]
    pub onchange: Callback<(String, String)>,
    // pub extra_class: String,
}

/// UnControlled Input Component
#[function_component(Input)]
pub fn input(props: &Props) -> Html {
    let Props {
        name,
        kind,
        title,
        placeholder,
        onchange,
        // extra_class,
    } = props.clone();

    let onchange = {
        let name = name.clone();
        Callback::from(move |e: Event| onchange.emit((name.clone(), get_value_from_event(e))))
    };

    html! {
        <div class="relative border border-gray-300 rounded-md px-3 py-2 shadow-sm focus-within:ring-1 focus-within:ring-indigo-600 focus-within:border-indigo-600">
                <label for={name.clone()} class="absolute -top-2 left-2 -mt-px inline-block px-1 bg-white text-xs font-medium text-gray-900">{title}</label>
                <input type={kind} id={name.clone()} {name} {placeholder} {onchange} class="block w-full border-0 p-0 text-gray-900 placeholder-gray-500 focus:ring-0 sm:text-sm" />
            </div>
    }
}
