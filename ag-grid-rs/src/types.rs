use std::future::Future;

use ag_grid_derive::FromInterface;
use js_sys::Function;
use serde::Serialize;
use serde_wasm_bindgen::Serializer as WasmSerializer;
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::spawn_local;

use crate::{utils::log, RowData};

#[wasm_bindgen]
extern "C" {
    type IGetRowsParams;

    #[wasm_bindgen(method, getter, js_name = startRow)]
    fn start_row(this: &IGetRowsParams) -> u32;

    #[wasm_bindgen(method, getter, js_name = endRow)]
    fn end_row(this: &IGetRowsParams) -> u32;

    #[wasm_bindgen(method, getter, js_name = sortModel)]
    fn sort_model(this: &IGetRowsParams) -> Vec<ISortModelItem>;

    #[wasm_bindgen(method, getter, js_name = filterModel)]
    fn filter_model(this: &IGetRowsParams) -> JsValue;

    #[wasm_bindgen(method, getter, js_name = successCallback)]
    fn success_callback(this: &IGetRowsParams) -> Function;

    #[wasm_bindgen(method, getter, js_name = failCallback)]
    fn fail_callback(this: &IGetRowsParams) -> Function;
}

#[derive(FromInterface)]
pub struct GetRowsParams {
    pub start_row: u32,
    pub end_row: u32,
    pub sort_model: std::vec::Vec<SortModelItem>,
}

#[wasm_bindgen]
extern "C" {
    type ISortModelItem;

    #[wasm_bindgen(method, getter, js_name = colId)]
    fn col_id(this: &ISortModelItem) -> String;

    #[wasm_bindgen(method, getter)]
    fn sort(this: &ISortModelItem) -> SortDirection;
}

#[derive(Debug, FromInterface)]
pub struct SortModelItem {
    pub col_id: String,
    pub sort: SortDirection,
}

#[derive(Debug)]
#[wasm_bindgen]
pub enum SortDirection {
    Asc,
    Desc,
}

#[wasm_bindgen]
pub struct DataSource {
    #[wasm_bindgen(readonly, getter_with_clone, js_name = getRows)]
    pub get_rows: Function,
}

/// Builder for the datasource used by both `PaginationController` and
/// `InfiniteRowModel`.
pub struct DataSourceBuilder {
    /// Callback the grid calls that you implement to fetch rows from the
    /// server.
    get_rows: Closure<dyn FnMut(IGetRowsParams)>,
    // Missing: optional "destroy" method
}

impl DataSourceBuilder {
    /// Start constructing a new `DataSourceBuilder` by providing a callback
    /// function which will receive `GetRowsParameters`. This callback is
    /// called by AG Grid to request new rows from the server.
    pub fn new<F, Fut>(mut get_rows: F) -> Self
    where
        F: FnMut(GetRowsParams) -> Fut + 'static,
        Fut: Future<Output = Result<Vec<RowData>, Box<dyn std::error::Error>>> + 'static,
    {
        let get_rows = Closure::<dyn FnMut(IGetRowsParams)>::new(move |js_params| {
            let params = GetRowsParams::from(&js_params);
            let fut = get_rows(params);

            let wrapper = async move {
                match fut.await {
                    Ok(data) => {
                        let data = data.serialize(&WasmSerializer::json_compatible()).unwrap();
                        js_params
                            .success_callback()
                            .call1(&JsValue::null(), &data)
                            .expect("failed calling success callback");
                    }
                    Err(e) => {
                        log(format!("Error calling get_rows callback: {e:?}"));
                        js_params
                            .fail_callback()
                            .call0(&JsValue::null())
                            .expect("failed calling failure callback");
                    }
                };
            };

            spawn_local(wrapper)
        });

        Self { get_rows }
    }

    pub fn build(self) -> DataSource {
        DataSource {
            get_rows: self.get_rows.into_js_value().unchecked_into(),
        }
    }
}

/// Options for the row model type.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub enum RowModelType {
    Infinite,
    Viewport,
    ClientSide,
    ServerSide,
}

/// Pre-specified filters which can be applied to columns.
// #[derive(Serialize)]
// #[serde(rename_all = "camelCase")]
pub enum Filter {
    /// A filter for number comparisons.
    AgNumberColumnFilter,
    /// A filter for string comparisons.
    AgTextColumnFilter,
    /// A filter for date comparisons.
    AgDateColumnFilter,
    /// A filter influenced by how filters work in Microsoft Excel. This is an
    /// AG Grid Enterprise feature.
    AgSetColumnFilter,
    /// Enable the default filter. The default is Text Filter for AG Grid
    /// Community and Set Filter for AG Grid Enterprise.
    // #[serde(serialize_with = "serialize_true")]
    True,
    /// Explicitly disable filtering.
    // #[serde(serialize_with = "serialize_false")]
    False,
    // TODO: Custom(FilterComponent)
}

impl Serialize for Filter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match *self {
            Filter::AgNumberColumnFilter => serializer.serialize_str("agNumberColumnFilter"),
            Filter::AgTextColumnFilter => serializer.serialize_str("agTextColumnFilter"),
            Filter::AgDateColumnFilter => serializer.serialize_str("agDateColumnFilter"),
            Filter::AgSetColumnFilter => serializer.serialize_str("agSetColumnFilter"),
            Filter::True => serializer.serialize_bool(true),
            Filter::False => serializer.serialize_bool(false),
        }
    }
}

/// An enumeration of possible values for [`ColumnDef::lock_position`].
pub enum LockPosition {
    True,
    False,
    Left,
    Right,
}

impl Serialize for LockPosition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match *self {
            LockPosition::True => serializer.serialize_bool(true),
            LockPosition::False => serializer.serialize_bool(false),
            LockPosition::Left => serializer.serialize_str("left"),
            LockPosition::Right => serializer.serialize_str("right"),
        }
    }
}

/// An enumeration of possible values for [`ColumnDef::pinned`] and
/// [`ColumnDef::initial_pinned`].
pub enum PinnedPosition {
    True,
    False,
    Left,
    Right,
}

impl Serialize for PinnedPosition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match *self {
            PinnedPosition::True => serializer.serialize_bool(true),
            PinnedPosition::False => serializer.serialize_bool(false),
            PinnedPosition::Left => serializer.serialize_str("left"),
            PinnedPosition::Right => serializer.serialize_str("right"),
        }
    }
}

/// An enumeration of possible values for
/// [`ColumnDef::set_editor_popup_position`].
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub enum PopupPosition {
    Over,
    Under,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub enum MenuTab {
    FilterMenuTab,
    GeneralMenuTab,
    ColumnsMenuTab,
}

// TODO: shouldn't we need a trait bound "T: Serialize"?
#[derive(Serialize)]
#[serde(untagged)]
pub(crate) enum OneOrMany<T> {
    One(T),
    Many(Vec<T>),
}

impl From<String> for OneOrMany<String> {
    fn from(v: String) -> Self {
        Self::One(v)
    }
}

impl<T> From<Vec<T>> for OneOrMany<T>
where
    T: Into<OneOrMany<T>>,
{
    fn from(v: Vec<T>) -> Self {
        Self::Many(v)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{json, to_value};

    use super::*;

    #[test]
    #[rustfmt::skip]
    fn test_serialize_filter() {
        assert_eq!(to_value(Filter::AgNumberColumnFilter).unwrap(), json!("agNumberColumnFilter"));
        assert_eq!(to_value(Filter::AgTextColumnFilter).unwrap(), json!("agTextColumnFilter"));
        assert_eq!(to_value(Filter::AgDateColumnFilter).unwrap(), json!("agDateColumnFilter"));
        assert_eq!(to_value(Filter::AgSetColumnFilter).unwrap(), json!("agSetColumnFilter"));
        assert_eq!(to_value(Filter::True).unwrap(), json!(true));
        assert_eq!(to_value(Filter::False).unwrap(), json!(false));
    }

    #[test]
    #[rustfmt::skip]
    fn test_serialize_row_model_type() {
        assert_eq!(to_value(RowModelType::Infinite).unwrap(), json!("infinite"));
        assert_eq!(to_value(RowModelType::Viewport).unwrap(), json!("viewport"));
        assert_eq!(to_value(RowModelType::ClientSide).unwrap(), json!("clientSide"));
        assert_eq!(to_value(RowModelType::ServerSide).unwrap(), json!("serverSide"));
    }
}
