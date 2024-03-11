use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub loading: bool,
    #[prop_or_default]
    pub disabled: bool,
    pub onclick: Callback<MouseEvent>,
    pub children: Children,
}

#[function_component(Button)]
pub fn button(props: &Props) -> Html {
    let Props {
        loading,
        disabled,
        onclick,
        children,
    } = props;

    html! {
      <button {onclick} class={classes!("btn-filled", disabled.then(|| "btn-disabled"))} disabled={*disabled}>
        if *loading {
            <div class="flex ml-2 items-center">
                <p>{"Processing..."}</p>
            </div>
        } else {
            // {for children.iter()}
            {children.clone()}
        }
     </button>
    }
}
