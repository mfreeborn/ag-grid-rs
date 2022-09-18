use ag_grid_rs::{traits::ToJsValue, ColumnDef};
use wasm_bindgen::{prelude::*, JsCast, JsValue};
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn test_serialize_column() {
    let col = ColumnDef::new("make").to_js_value();

    assert_eq!(to_obj(col).get("field").as_string().unwrap(), "make");
}

#[wasm_bindgen]
extern "C" {
    pub type Object;

    #[wasm_bindgen(method, indexing_setter)]
    pub fn set(this: &Object, key: &str, value: JsValue);

    #[wasm_bindgen(method, indexing_getter)]
    pub fn get(this: &Object, key: &str) -> JsValue;
}

impl Object {
    pub fn new() -> Self {
        js_sys::Object::new().unchecked_into::<Self>()
    }
}

impl Default for Object {
    fn default() -> Self {
        Self::new()
    }
}

fn to_obj(value: JsValue) -> Object {
    value.unchecked_into::<Object>()
}
