use gloo_utils::format::JsValueSerdeExt;
use serde_json::Result;
use wasm_bindgen::JsValue;

/// #[derive(Serialize, Deserialize)]
/// pub struct Example {
///     pub field1: Vec<Vec<f32>>,
///     pub field2: [f32; 4],
/// }
///
/// #[wasm_bindgen]
/// pub fn send_example_to_js() -> JsValue {
///     let example = Example {
///         field1: vec![vec![1., 2.], vec![3., 4.]],
///         field2: [1., 2., 3., 4.],
///     };
///     to_js(&example).unwrap()
/// }
///
/// #[wasm_bindgen]
/// pub fn receive_example_from_js(val: JsValue) {
///     let _example: Example = from_js(&val).unwrap();
/// }
pub fn from_js<T: for<'a> serde::de::Deserialize<'a>>(val: &JsValue) -> Result<T> {
    JsValueSerdeExt::into_serde::<T>(val)
}

pub fn to_js<T: serde::ser::Serialize + ?Sized>(t: &T) -> Result<JsValue> {
    <JsValue as JsValueSerdeExt>::from_serde(t)
}
