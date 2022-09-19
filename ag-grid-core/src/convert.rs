//! Access to the `ToJsValue` trait for converting types into
//! `wasm_bindgen::JsValue`s.

use std::collections::HashMap;

use js_sys::Array;
use wasm_bindgen::{prelude::Closure, JsValue};

use crate::imports::Object;

/// This trait is used to provide an implementation for converting a given type
/// into a `wasm_bindgen::JsValue`.
pub trait ToJsValue {
    /// Convert the current type to a `wasm_bindgen::JsValue`;
    fn to_js_value(&self) -> JsValue;
}

impl ToJsValue for String {
    fn to_js_value(&self) -> JsValue {
        JsValue::from_str(self)
    }
}

impl ToJsValue for &'static str {
    fn to_js_value(&self) -> JsValue {
        JsValue::from_str(self)
    }
}

impl ToJsValue for bool {
    fn to_js_value(&self) -> JsValue {
        JsValue::from_bool(*self)
    }
}

// It would be a nice exercise to write a macro for these...

impl ToJsValue for usize {
    fn to_js_value(&self) -> JsValue {
        JsValue::from_f64(*self as f64)
    }
}

impl ToJsValue for isize {
    fn to_js_value(&self) -> JsValue {
        JsValue::from_f64(*self as f64)
    }
}

impl ToJsValue for u64 {
    fn to_js_value(&self) -> JsValue {
        JsValue::from_f64(*self as f64)
    }
}

impl ToJsValue for i64 {
    fn to_js_value(&self) -> JsValue {
        JsValue::from_f64(*self as f64)
    }
}

impl ToJsValue for u32 {
    fn to_js_value(&self) -> JsValue {
        JsValue::from_f64(*self as f64)
    }
}

impl ToJsValue for i32 {
    fn to_js_value(&self) -> JsValue {
        JsValue::from_f64(*self as f64)
    }
}

impl ToJsValue for u16 {
    fn to_js_value(&self) -> JsValue {
        JsValue::from_f64(*self as f64)
    }
}

impl ToJsValue for i16 {
    fn to_js_value(&self) -> JsValue {
        JsValue::from_f64(*self as f64)
    }
}

impl ToJsValue for u8 {
    fn to_js_value(&self) -> JsValue {
        JsValue::from_f64(*self as f64)
    }
}

impl ToJsValue for i8 {
    fn to_js_value(&self) -> JsValue {
        JsValue::from_f64(*self as f64)
    }
}

impl ToJsValue for f32 {
    fn to_js_value(&self) -> JsValue {
        JsValue::from_f64(*self as f64)
    }
}

impl ToJsValue for f64 {
    fn to_js_value(&self) -> JsValue {
        JsValue::from_f64(*self as f64)
    }
}

impl<T> ToJsValue for Option<T>
where
    T: ToJsValue,
{
    fn to_js_value(&self) -> JsValue {
        match self {
            Some(val) => val.to_js_value(),
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

impl ToJsValue for () {
    fn to_js_value(&self) -> JsValue {
        JsValue::undefined()
    }
}
