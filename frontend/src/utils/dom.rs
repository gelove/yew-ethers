use log::info;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::{window, Document, Element, HtmlInputElement, InputEvent};
use yew::{events::Event, TargetCast};

pub fn document() -> Document {
    let window = window().expect_throw("no global `window` exists");

    window
        .document()
        .expect_throw("should have a document on window")
}

pub fn get_id(id: &str) -> Element {
    document()
        .get_element_by_id(id)
        .expect_throw(&format!("find {}", id))
}

pub fn get_value_from_event(e: Event) -> String {
    // let target = e.target_dyn_into::<HtmlInputElement>();
    // target.map(|target| target.value()).unwrap_throw()
    let target = e.target_dyn_into::<HtmlInputElement>().unwrap_throw();
    target.value()
}

pub fn get_value_from_input_event(e: InputEvent) -> String {
    // let event: Event = e.dyn_into().unwrap_throw();
    // let event_target = event.target().unwrap_throw();
    // let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
    let target = e.target_unchecked_into::<HtmlInputElement>();
    info!("get_value_from_input_event => {}", target.value());
    target.value()
}
