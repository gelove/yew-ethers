use yew::prelude::*;

pub struct Button;

#[derive(PartialEq, Properties, Clone)]
pub struct Props {
    pub txt: String,
    pub callback: Callback<()>,
}

pub enum Msg {
    Click,
}

impl Component for Button {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Click => {
                ctx.props().callback.emit(());
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <button class={"button"} onclick={ctx.link().callback(|_| Msg::Click)}>{ ctx.props().txt.clone() }</button>
        }
    }
}
