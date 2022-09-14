use serde::Serializer;

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
