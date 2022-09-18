use std::future::Future;

use ag_grid_derive::{FromInterface, ToJsValue};
use js_sys::Function;
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::spawn_local;

use crate::{traits::ToJsValue as ToJsValueTrait, utils::log, RowData};

#[wasm_bindgen]
extern "C" {
    pub(crate) type IGetRowsParams;

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

#[wasm_bindgen]
extern "C" {
    pub(crate) type IHeaderValueGetterParams;

    #[wasm_bindgen(method, getter, js_name = colId)]
    pub(crate) fn location(this: &IHeaderValueGetterParams) -> Option<String>;
}

#[derive(Debug, FromInterface)]
pub struct HeaderValueGetterParams {
    pub location: Option<String>,
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
        Fut: Future<Output = Result<(Vec<RowData>, Option<u32>), Box<dyn std::error::Error>>>
            + 'static,
    {
        let get_rows =
            Closure::<dyn FnMut(IGetRowsParams)>::new(move |js_params: IGetRowsParams| {
                let params = (&js_params).into();
                let fut = get_rows(params);

                let wrapper = async move {
                    match fut.await {
                        Ok((data, last_row_index)) => {
                            let data = data.to_js_value();
                            let last_row_index = last_row_index.to_js_value();
                            js_params
                                .success_callback()
                                .call2(&JsValue::null(), &data, &last_row_index)
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

/// An enumeration of possible sorting methods.
#[derive(ToJsValue)]
pub enum SortMethod {
    Asc,
    Desc,
    // #[serde(serialize_with = "crate::utils::serialize_null")]
    Null,
}

/// Options for the row model type.
#[derive(ToJsValue)]
pub enum RowModelType {
    Infinite,
    Viewport,
    ClientSide,
    ServerSide,
}

/// Pre-specified filters which can be applied to columns.
#[derive(ToJsValue)]
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

/// An enumeration of possible values for [`ColumnDef::lock_position`].
#[derive(ToJsValue)]
pub enum LockPosition {
    True,
    False,
    Left,
    Right,
}

/// An enumeration of possible values for [`ColumnDef::pinned`] and
/// [`ColumnDef::initial_pinned`].
#[derive(ToJsValue)]
pub enum PinnedPosition {
    True,
    False,
    Left,
    Right,
}

/// An enumeration of possible values for
/// [`ColumnDef::set_editor_popup_position`].
#[derive(ToJsValue)]
pub enum PopupPosition {
    Over,
    Under,
}

#[derive(ToJsValue)]
pub enum MenuTab {
    FilterMenuTab,
    GeneralMenuTab,
    ColumnsMenuTab,
}

pub(crate) enum OneOrMany<T>
where
    T: ToJsValueTrait,
{
    One(T),
    Many(Vec<T>),
}

impl<T> OneOrMany<T>
where
    T: ToJsValueTrait,
{
    pub(crate) fn to_js_value(&self) -> JsValue {
        match self {
            Self::One(v) => v.to_js_value(),
            Self::Many(v) => v.to_js_value(),
        }
    }
}

impl<T> From<T> for OneOrMany<T>
where
    T: ToJsValueTrait,
{
    fn from(v: T) -> Self {
        Self::One(v)
    }
}

impl<T> From<Vec<T>> for OneOrMany<T>
where
    T: Into<OneOrMany<T>> + ToJsValueTrait,
{
    fn from(v: Vec<T>) -> Self {
        Self::Many(v)
    }
}
