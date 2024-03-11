use yew::prelude::*;

#[derive(PartialEq, PartialOrd)]
pub enum State {
    Info,
    Success,
    Error,
}

impl State {
    pub fn to_string(&self) -> String {
        match self {
            State::Info => "bg-blue-400".to_owned(),
            State::Success => "bg-green-400".to_owned(),
            State::Error => "bg-red-400".to_owned(),
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: String,
    pub state: State,
    pub on_load: Callback<String>,
}

#[function_component(MainTitle)]
pub fn main_title(props: &Props) -> Html {
    let Props {
        title,
        state,
        on_load,
    } = props;

    on_load.emit("main_title loading".to_owned());

    html! {
        <h1 class={state.to_string()}>{title}</h1>
    }
}
