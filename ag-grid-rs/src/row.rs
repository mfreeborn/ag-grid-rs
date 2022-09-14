use std::collections::HashMap;

use serde::Serialize;
use serde_json::Value;

use crate::utils::IntoValue;

#[derive(Serialize)]
pub struct RowData {
    #[serde(flatten)]
    data: HashMap<String, Value>,
}

impl RowData {
    pub fn new<F>(data: Vec<(F, &dyn IntoValue)>) -> Self
    where
        F: AsRef<str>,
    {
        Self {
            data: data
                .into_iter()
                .map(|(field, val)| (field.as_ref().to_string(), val.into_value()))
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{json, to_value};

    use super::*;

    #[test]
    fn test_serialize_row_data() {
        let row = RowData::new(vec![
            ("make".to_string(), &"Porche"),
            ("model".to_string(), &"Cayenne"),
            ("price".to_string(), &50000),
        ]);
        let expected = json!({"make": "Porche", "model": "Cayenne", "price": 50000});

        assert_eq!(to_value(row).unwrap(), expected);
    }
}
