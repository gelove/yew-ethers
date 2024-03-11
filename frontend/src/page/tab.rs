use crate::component::button::Button;
use yew::prelude::*;

pub enum Msg {
    AddOne,
    AddTwo,
}

pub struct Tab {
    item_type: i32,
}

impl Tab {
    fn btn_click(&mut self) {
        // js_api::js_alert("hello from wasm!");
        self.item_type = 1;
    }

    fn btn_two_click(&mut self) {
        self.item_type = 0;
    }

    fn hello_world_ui(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
            <center>
                <h1>{"布局以被切换"}</h1>
                <Button onclick={ctx.link().callback(|_| Msg::AddTwo)}>{"点击 上一个布局"}</Button>
            </center>
            </div>
        }
    }

    fn def_ui(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
            <center>
                <h1>{ "测试实例一" }</h1>
                <Button onclick={ctx.link().callback(|_| Msg::AddOne)}>{"点击 下一个布局"}</Button>
            </center>
            </div>
        }
    }
}

impl Component for Tab {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { item_type: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.btn_click();
                true
            }
            Msg::AddTwo => {
                self.btn_two_click();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match self.item_type {
            0 => self.def_ui(ctx),
            _ => self.hello_world_ui(ctx),
        }
    }
}
