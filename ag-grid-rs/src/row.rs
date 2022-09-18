use std::collections::HashMap;

use wasm_bindgen::JsValue;

use crate::traits::ToJsValue;

pub struct RowData {
    data: HashMap<String, JsValue>,
}

impl RowData {
    pub fn new<F>(data: Vec<(F, &dyn ToJsValue)>) -> Self
    where
        F: AsRef<str>,
    {
        Self {
            data: data
                .into_iter()
                .map(|(field, val)| (field.as_ref().to_string(), val.to_js_value()))
                .collect(),
        }
    }
}

impl ToJsValue for RowData {
    fn to_js_value(&self) -> JsValue {
        self.data.to_js_value()
    }
}
