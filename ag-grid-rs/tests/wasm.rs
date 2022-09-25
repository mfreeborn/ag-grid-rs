use ag_grid_core::{convert::ToJsValue, imports::ObjectExt};
use ag_grid_rs::{column::SortMethod, ColumnDef, GridOptions, ToJsValue};
use js_sys::Array;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn test_serialize_sort_method() {
    assert_eq!(SortMethod::Asc.to_js_value().as_string().unwrap(), "asc");
    assert_eq!(SortMethod::Desc.to_js_value().as_string().unwrap(), "desc");
    assert!(SortMethod::Null.to_js_value().is_null());
}

#[wasm_bindgen_test]
fn test_serialize_grid_options() {
    #[derive(ToJsValue)]
    struct Data {
        make: String,
        model: String,
        price: u32,
    }

    let row = Data {
        make: "Jaguar".to_string(),
        model: "F-Type".to_string(),
        price: 100_000,
    };

    let grid_options = GridOptions::new().row_data(vec![row]).to_js_value();

    let obj = to_obj(&grid_options);
    assert!(Array::is_array(&obj.get("rowData")));
    let row_data = obj.get("rowData").unchecked_into::<Array>();
    for r in row_data.iter() {
        let row = r.unchecked_into::<ObjectExt>();
        assert_eq!(row.get("make").as_string().unwrap(), "Jaguar");
        assert_eq!(row.get("model").as_string().unwrap(), "F-Type");
        assert_eq!(row.get("price").as_f64().unwrap(), 100000f64);
    }
}

#[wasm_bindgen_test]
fn test_serialize_column() {
    let col = ColumnDef::new("make").to_js_value();

    assert_eq!(to_obj(&col).get("field").as_string().unwrap(), "make");
}

fn to_obj(value: &JsValue) -> ObjectExt {
    value.to_owned().unchecked_into::<ObjectExt>()
}
