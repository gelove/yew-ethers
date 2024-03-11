use log::info;
use std::ops::Deref;
use wasm_bindgen::prelude::*;
use web_sys::HtmlAnchorElement;
use yew::prelude::*;
use yew::virtual_dom::{VNode, VTag};
use yew_router::prelude::*;

use crate::component::svg;
use crate::constant::router::*;

fn navigate(nav: String) -> Route {
    match nav {
        nav if nav == DASHBOARD => Route::Home,
        nav if nav == AIRDROP => Route::Airdrop,
        nav if nav == PRESALE => Route::Presale,
        nav if nav == CALENDAR => Route::Calendar,
        nav if nav == DOCUMENTS => Route::Documents,
        nav if nav == REPORTS => Route::Reports,
        _ => Route::Home,
    }
}

fn nav_item(current: String, data: &(&str, fn(bool) -> VTag, Callback<MouseEvent>)) -> Html {
    let (title, svg, onclick) = data;
    let activated = *title == &current;
    let class = if activated {
        "active group"
    } else {
        "normal group"
    };

    html! {
        <a href="#" {class} {onclick} name={String::from(*title)}>
            {VNode::from(svg(activated))}
            {*title}
        </a>
    }
}

#[function_component(Sidebar)]
pub fn sidebar() -> Html {
    let current = use_state_eq(|| String::from(DASHBOARD));

    let history = use_history().unwrap();

    let onclick = {
        let current = current.clone();
        let history = history.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            // let target = e.target_dyn_into::<HtmlElement>().unwrap_throw();
            // let map = target.dataset();
            // let nav = map.get("nav").unwrap_throw();
            let target = e.target_dyn_into::<HtmlAnchorElement>().unwrap_throw();
            let nav = target.name();
            current.set(nav.clone());
            let route = navigate(nav.clone());
            info!("nav => route : {:?} => {:?}", nav, route);
            history.push(route);
        })
    };

    let nav_list: Vec<(&str, fn(bool) -> VTag, Callback<MouseEvent>)> = vec![
        (DASHBOARD, svg::home, onclick.clone()),
        (AIRDROP, svg::users, onclick.clone()),
        (PRESALE, svg::folder, onclick.clone()),
        (CALENDAR, svg::calendar, onclick.clone()),
        (DOCUMENTS, svg::inbox, onclick.clone()),
        (REPORTS, svg::chart_bar, onclick.clone()),
    ];

    html! {
        <>
        // 手机浏览器
        <div class="fixed inset-0 flex z-40 md:hidden" role="dialog" aria-modal="true">
            <div class="fixed inset-0 bg-gray-600 bg-opacity-75" aria-hidden="true"></div>
            <div class="relative flex-1 flex flex-col max-w-xs w-full pt-5 pb-4 bg-gray-800">
            <div class="absolute top-0 right-0 -mr-12 pt-2">
                <button type="button" class="ml-1 flex items-center justify-center h-10 w-10 rounded-full focus:outline-none focus:ring-2 focus:ring-inset focus:ring-white">
                <span class="sr-only">{"Close sidebar"}</span>
                <svg class="h-6 w-6 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" aria-hidden="true">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                </svg>
                </button>
            </div>
            <div class="flex justify-center items-center flex-shrink-0 flex items-center px-4">
                <img class="h-8 w-auto" src="asset/image/avatar.jpeg" alt="CoinTools" />
                <span class="ml-4 text-white">{"CoinTools"}</span>
            </div>
            <div class="mt-5 flex-1 h-0 overflow-y-auto">
                <nav class="px-2 space-y-1">
                    {nav_list.iter().map(|data| nav_item(current.deref().clone(), data)).collect::<Html>()}
                </nav>
            </div>
            </div>
            <div class="flex-shrink-0 w-14" aria-hidden="true"></div>
        </div>

        // 桌面浏览器
        <div class="hidden md:flex md:flex-col md:w-64 md:h-screen">
            <div class="flex-1 flex flex-col min-h-0 bg-gray-800">
            <div class="flex justify-center items-center h-16 flex-shrink-0 px-4 bg-gray-900">
                <img class="h-8 w-auto" src="asset/image/avatar.jpeg" alt="CoinTools" />
                <span class="ml-4 text-white">{"CoinTools"}</span>
            </div>

            <div class="flex-1 flex flex-col overflow-y-auto">
                <nav class="flex-1 px-2 py-4 space-y-1">
                    {nav_list.iter().map(|data| nav_item(current.deref().clone(), data)).collect::<Html>()}
                </nav>
            </div>
            </div>
        </div>
        </>
    }
}
