use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
      <div class="py-6">
        <div class="max-w-7xl mx-auto px-4 sm:px-6 md:px-8">
          <h1 class="text-2xl font-semibold text-gray-900">{"Dashboard"}</h1>
        </div>
        <div class="max-w-7xl mx-auto py-4 px-4 sm:px-6 md:px-8">
            <div class="flex justify-around border-4 border-dashed border-gray-200 rounded-lg h-96">
                {view_card("Rust", Some("asset/image/rust.svg"), html! {
                    <p>{"Rust is a modern systems programming language focusing on safety, speed, and concurrency."}</p>
                })}
                {view_card("Yew", Some("asset/image/yew.svg"), html! {
                    <p>{"Yew is a modern Rust framework for creating multi-threaded front-end web apps with WebAssembly."}</p>
                })}
                {view_card("Ethers-rs", Some("asset/image/ethers.svg"), html! {
                    <p>{"Ethers-rs is complete Ethereum and Celo wallet implementation and utilities in Rust."}</p>
                })}
            </div>
        </div>
      </div>
    }
}

fn view_card(title: &'static str, img_url: Option<&'static str>, content: Html) -> Html {
    html! {
        <div class="w-80 h-48 rounded bg-blue-400 text-white p-6">
            {for img_url.map(|url| html! {
                <img class="float-right h-12 w-auto" src={url} alt="Logo" />
            })}
            <h1 class="text-3xl mb-8">{title}</h1>
            {content}
        </div>
    }
}
