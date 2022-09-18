use ag_grid_rs::{traits::ToJsValue, utils::Object, ColumnDef, GridOptions, RowData};
use js_sys::Array;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn test_serialize_grid_options() {
    let row = RowData::new(vec![
        ("make", &"Jaguar"),
        ("model", &"F-Type"),
        ("price", &100_000),
    ]);

    let grid_options = GridOptions::new().row_data(vec![row]).to_js_value();

    let obj = to_obj(&grid_options);
    assert!(Array::is_array(&obj.get("rowData")));
    let row_data = obj.get("rowData").unchecked_into::<Array>();
    for r in row_data.iter() {
        let row = r.unchecked_into::<Object>();
        assert_eq!(row.get("make").as_string().unwrap(), "Jaguar");
        assert_eq!(row.get("model").as_string().unwrap(), "F-Type");
        assert_eq!(row.get("price").as_f64().unwrap(), 100000f64);
    }
}

#[wasm_bindgen_test]
fn test_serialize_row_data() {
    let row = RowData::new(vec![
        ("make", &"Jaguar"),
        ("model", &"F-Type"),
        ("price", &100_000),
    ])
    .to_js_value();

    assert_eq!(to_obj(&row).get("make").as_string().unwrap(), "Jaguar");
    assert_eq!(to_obj(&row).get("model").as_string().unwrap(), "F-Type");
    assert_eq!(to_obj(&row).get("price").as_f64().unwrap(), 100_000f64);
}

#[wasm_bindgen_test]
fn test_serialize_column() {
    let col = ColumnDef::new("make").to_js_value();

    assert_eq!(to_obj(&col).get("field").as_string().unwrap(), "make");
}

fn to_obj(value: &JsValue) -> Object {
    value.to_owned().unchecked_into::<Object>()
}
