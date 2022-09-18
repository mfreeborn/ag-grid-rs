use std::collections::HashMap;

use js_sys::Array;
use wasm_bindgen::{prelude::Closure, JsValue};

use crate::utils::Object;

pub trait ToJsValue {
    fn to_js_value(&self) -> JsValue;
}

impl ToJsValue for String {
    fn to_js_value(&self) -> JsValue {
        JsValue::from_str(self)
    }
}

impl ToJsValue for bool {
    fn to_js_value(&self) -> JsValue {
        JsValue::from_bool(*self)
    }
}

impl ToJsValue for u32 {
    fn to_js_value(&self) -> JsValue {
        JsValue::from_f64(*self as f64)
    }
}

impl ToJsValue for Option<u32> {
    fn to_js_value(&self) -> JsValue {
        match *self {
            Some(val) => val.into(),
            None => JsValue::null(),
        }
    }
}

impl<T> ToJsValue for Vec<T>
where
    T: ToJsValue,
{
    fn to_js_value(&self) -> JsValue {
        self.iter()
            .map(|v| v.to_js_value())
            .collect::<Array>()
            .into()
    }
}

impl<V> ToJsValue for HashMap<String, V>
where
    V: ToJsValue,
{
    fn to_js_value(&self) -> JsValue {
        let obj = Object::new();
        for (k, v) in self {
            obj.set(k, v.to_js_value())
        }
        obj.into()
    }
}

impl<T> ToJsValue for Closure<T>
where
    T: ?Sized,
{
    fn to_js_value(&self) -> JsValue {
        // I'm not sure if this messes up the memory management of `Closure`. Normally,
        // to release it to the JS side completely, `Closure::into_js_value()` is
        // called, which drops the (owned) `Closure` in Rust and returns just the
        // `JsValue`. Here we are getting a `JsValue`, but we are only working
        // on a reference to the `Closure`.
        self.as_ref().to_owned()
    }
}

impl ToJsValue for JsValue {
    fn to_js_value(&self) -> JsValue {
        self.to_owned()
    }
}
