use wasm_bindgen::prelude::*;
use web_sys::{Event, File, KeyboardEvent, MouseEvent};

#[wasm_bindgen(module = "/asset/js/editor/index.js")]
extern "C" {
    #[wasm_bindgen(js_name = getContent)]
    pub fn get_content() -> String;
    #[wasm_bindgen(js_name = inputTag)]
    pub fn input_tag(event: KeyboardEvent);
    #[wasm_bindgen(js_name = showOriginTags)]
    pub fn show_origin_tags(tags: Vec<JsValue>);
    #[wasm_bindgen(js_name = getAddedTags)]
    pub fn get_added_tags() -> Vec<JsValue>;
    #[wasm_bindgen(js_name = randomTitleImage)]
    pub fn random_title_image(event: MouseEvent, id: u64, payload_callback: JsValue);
    #[wasm_bindgen(js_name = uploadTitleImage)]
    pub fn upload_title_image(
        event: Event,
        post_id: u64,
        files: Vec<File>,
        payload_callback: JsValue,
    );
}
