#![recursion_limit = "512"]

use frontend::router::App;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
