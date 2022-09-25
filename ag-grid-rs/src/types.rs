//! Generally internal types for the ag-grid-rs library.

use ag_grid_core::convert::ToJsValue;
use wasm_bindgen::JsValue;

pub(crate) enum OneOrMany<T>
where
    T: ToJsValue,
{
    One(T),
    Many(Vec<T>),
}

impl<T> ToJsValue for OneOrMany<T>
where
    T: ToJsValue,
{
    fn to_js_value(&self) -> JsValue {
        match self {
            Self::One(v) => v.to_js_value(),
            Self::Many(v) => v.to_js_value(),
        }
    }
}

impl<T> From<T> for OneOrMany<T>
where
    T: ToJsValue,
{
    fn from(v: T) -> Self {
        Self::One(v)
    }
}

impl<T> From<Vec<T>> for OneOrMany<T>
where
    T: Into<OneOrMany<T>> + ToJsValue,
{
    fn from(v: Vec<T>) -> Self {
        Self::Many(v)
    }
}
