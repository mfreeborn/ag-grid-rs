use serde::Serializer;
use wasm_bindgen::prelude::*;

#[allow(unused)]
pub fn serialize_true<S>(serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_bool(true)
}

#[allow(unused)]
pub fn serialize_false<S>(serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_bool(false)
}

#[allow(clippy::wrong_self_convention)]
pub trait IntoValue {
    fn into_value(&self) -> serde_json::Value;
}

impl<T> IntoValue for T
where
    T: Into<serde_json::Value> + Clone,
{
    fn into_value(&self) -> serde_json::Value {
        self.to_owned().into()
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub(crate) fn log(x: String);
}

#[cfg(test)]
mod tests {
    use serde::Serialize;
    use serde_json::{json, to_value};

    use super::*;

    #[test]
    fn test_bool_serializer() {
        // TODO: this needs to pass without the use of #[serde(untagged)]
        #[derive(Serialize)]
        #[serde(untagged)]
        enum Test {
            #[serde(serialize_with = "serialize_true")]
            True,
            #[serde(serialize_with = "serialize_false")]
            False,
        }

        assert_eq!(to_value(Test::True).unwrap(), json!(true));
        assert_eq!(to_value(Test::False).unwrap(), json!(false));
    }
}
