use js_sys::Object;
use wasm_bindgen::{prelude::*, JsCast};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Object)]
    pub type ObjectExt;

    #[wasm_bindgen(method, indexing_setter)]
    pub fn set(this: &ObjectExt, key: &str, value: JsValue);

    #[wasm_bindgen(method, indexing_getter)]
    pub fn get(this: &ObjectExt, key: &str) -> JsValue;
}

impl ObjectExt {
    pub fn new() -> Self {
        Object::new().unchecked_into::<Self>()
    }

    pub fn get_string_unchecked<T: AsRef<str>>(&self, key: T) -> String {
        self.get(key.as_ref()).as_string().unwrap()
    }

    pub fn get_string<T: AsRef<str>>(&self, key: T) -> Option<String> {
        self.get(key.as_ref()).as_string()
    }

    pub fn get_f64_unchecked<T: AsRef<str>>(&self, key: T) -> f64 {
        self.get(key.as_ref()).as_f64().unwrap()
    }

    pub fn get_f64<T: AsRef<str>>(&self, key: T) -> Option<f64> {
        self.get(key.as_ref()).as_f64()
    }

    pub fn entries(&self) -> Vec<(String, JsValue)> {
        Object::entries(self)
            .iter()
            .map(|pair| {
                let pair = pair.unchecked_into::<js_sys::Array>();
                (pair.get(0).as_string().unwrap(), pair.get(1))
            })
            .collect()
    }

    pub fn keys(&self) -> Vec<String> {
        Object::keys(self)
            .iter()
            .map(|v| {
                // Safe to unwrap because object keys are always strings
                v.as_string().unwrap()
            })
            .collect()
    }

    pub fn values(&self) -> Vec<JsValue> {
        Object::values(self).iter().collect()
    }
}

impl Default for ObjectExt {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(v: &str);
}
