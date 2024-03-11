use log::info;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::component::button::Button;
use crate::component::title::{MainTitle, State};
use crate::constant::router::Route;

pub enum Msg {
    AddOne,
}

pub struct Model {
    value: i64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { value: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let hello = "Hello World!";
        let is_normal = true;
        let class_link =
            "px-4 py-2 hover:bg-black hover:text-white rounded border-black border".to_string();
        // let message = Some("I am a message!");
        let message: Option<&str> = None;

        let tasks = vec!["play basketball", "swimming", "running"];

        let main_title_load = Callback::from(|message: String| info!("{}", message));

        let history = ctx.link().history().unwrap_throw();
        // 只能执行一次
        let onclick = Callback::once(move |e: MouseEvent| {
            e.prevent_default();
            history.push(Route::Home);
        });

        html! {
            <div class="md:container md:mx-auto">
                // <a class="block bg-blue-400" href="/">{"Go Home"}</a>
                // <a class="block bg-blue-400" onclick={onclick.clone()}>{"Go Home"}</a>
                <Link<Route> to={Route::Home}><span class="block bg-green-400">{"Go Home"}</span></Link<Route>>
                <Button disabled={true} {onclick}>{"Go Home"}</Button>

                <MainTitle title={hello} state={State::Success} on_load={main_title_load} />

                <a target="_blank" class={classes!(&class_link, "block")} href="//www.google.com/">{"google"}</a>
                <a target="_blank" class={classes!(&class_link, is_normal.then(|| "block"))} href="//www.baidu.com/">{"baidu"}</a>

                if let Some(message) = message {
                    <p class="bg-red-400">{message}</p>
                } else {
                    <p class="bg-green-400">{"no message to see today"}</p>
                }

                <p>{"遍历"}</p>
                <ul>
                    {tasks.iter().map(|item| html!{<li class="bg-blue-400">{item}</li>}).collect::<Html>()}
                </ul>

                <p class="bg-red-400">{self.value}</p>
                <Button onclick={ctx.link().callback(|_| Msg::AddOne)}>{"Add One"}</Button>

                <div class="relative border border-gray-300 rounded-md px-3 py-2 shadow-sm focus-within:ring-1 focus-within:ring-indigo-600 focus-within:border-indigo-600">
                    <label for="task" class="absolute -top-2 left-2 -mt-px inline-block px-1 bg-white text-xs font-medium text-gray-900">{"Name"}</label>
                    <input type="text" name="name" id="name" class="block w-full border-0 p-0 text-gray-900 placeholder-gray-500 focus:ring-0 sm:text-sm" placeholder="Allen" />
                </div>
            </div>
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        true
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {}

    fn destroy(&mut self, _ctx: &Context<Self>) {}
}
