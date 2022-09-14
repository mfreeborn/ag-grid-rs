use serde::Serializer;

pub fn serialize_true<S>(serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_bool(true)
}

pub fn serialize_false<S>(serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_bool(false)
}

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
