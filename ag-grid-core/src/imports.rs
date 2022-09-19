use wasm_bindgen::prelude::*;

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
        use wasm_bindgen::JsCast;
        js_sys::Object::new().unchecked_into::<Self>()
    }
}

impl Default for Object {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(v: &str);
}
