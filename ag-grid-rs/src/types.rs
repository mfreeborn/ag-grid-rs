use std::future::Future;

use ag_grid_core::{convert::ToJsValue, imports::log};
use ag_grid_derive::{FromInterface, ToJsValue as ToJsValueMacro};
use js_sys::{Array, Function, Object};
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::spawn_local;

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
    fn filter_model(this: &IGetRowsParams) -> Object;

    #[wasm_bindgen(method, getter, js_name = successCallback)]
    fn success_callback(this: &IGetRowsParams) -> Function;

    #[wasm_bindgen(method, getter, js_name = failCallback)]
    fn fail_callback(this: &IGetRowsParams) -> Function;
}

/// Parameters passed to the callback function in [`DataSourceBuilder::new`].
#[derive(Debug)]
pub struct GetRowsParams {
    /// The first row index to get.
    pub start_row: u32,
    /// The first row index to *not* get.
    pub end_row: u32,
    /// A vector of `[SortModelItem]` describing how the data is expected to be
    /// sorted.
    pub sort_model: Vec<SortModelItem>,
    ///
    pub filter_model: String,
}

pub enum FilterModelType {
    Single(FilterModel),
    Combined(CombinedFilterModel),
}

pub enum FilterModel {
    Text(TextFilter),
    Number(NumberFilter),
    Date(DateFilter),
}

pub enum CombinedFilterModel {
    Text(CombinedTextFilter),
    Number(CombinedNumberFilter),
    Date(CombinedDateFilter),
}

pub enum JoinOperator {
    And,
    Or,
}

pub struct TextFilter {
    filter: Option<String>,
    filter_to: Option<String>,
    type_: Option<String>,
}

pub struct NumberFilter {
    filter: Option<u32>,
    filter_to: Option<u32>,
    type_: Option<String>,
}

pub struct DateFilter {
    filter: Option<String>,
    filter_to: Option<String>,
    type_: Option<String>,
}

pub struct CombinedTextFilter {
    condition_1: TextFilter,
    condition_2: TextFilter,
    operator: JoinOperator,
    type_: Option<String>,
}

pub struct CombinedNumberFilter {
    condition_1: NumberFilter,
    condition_2: NumberFilter,
    operator: JoinOperator,
    type_: Option<String>,
}

pub struct CombinedDateFilter {
    condition_1: DateFilter,
    condition_2: DateFilter,
    operator: JoinOperator,
    type_: Option<String>,
}

impl From<&IGetRowsParams> for GetRowsParams {
    fn from(i: &IGetRowsParams) -> Self {
        Self {
            start_row: i.start_row(),
            end_row: i.end_row(),
            sort_model: i.sort_model().iter().map(SortModelItem::from).collect(),
            filter_model: {
                let f = i.filter_model();

                //https://www.ag-grid.com/javascript-data-grid/filter-provided-simple/#simple-filter-options
                for filter in Object::entries(&f).iter() {
                    let pair = filter.unchecked_into::<Array>();
                    let col_name = pair.get(0).as_string().unwrap();
                    log(&col_name);

                    let filter_model = pair.get(1).unchecked_into::<Object>();
                    for f in Object::keys(&filter_model).iter() {
                        log(&f.as_string().unwrap());
                    }
                    if filter_model.has_own_property(&"operator".into()) {
                        log("combined")
                    } else {
                        log("single")
                    }
                }

                "hi".to_string()
            },
        }
    }
}

#[wasm_bindgen]
extern "C" {
    type ISortModelItem;

    #[wasm_bindgen(method, getter, js_name = colId)]
    fn col_id(this: &ISortModelItem) -> String;

    #[wasm_bindgen(method, getter)]
    fn sort(this: &ISortModelItem) -> SortDirection;
}

/// Details of how to sort the requested data.
#[derive(Debug, FromInterface)]
pub struct SortModelItem {
    /// Which column to sort.
    pub col_id: String,
    /// How the column should be sorted.
    pub sort: SortDirection,
}

#[wasm_bindgen]
extern "C" {
    pub(crate) type IHeaderValueGetterParams;

    #[wasm_bindgen(method, getter, js_name = colId)]
    pub(crate) fn location(this: &IHeaderValueGetterParams) -> Option<String>;
}

/// Parameters passed to the closure in
/// [`ColumnDef::header_value_getter`][`crate::ColumnDef::header_value_getter`].
#[derive(Debug, FromInterface)]
pub struct HeaderValueGetterParams {
    /// Where the column is going to appear.
    pub location: Option<String>,
}

/// Possible directions for which to sort data.
#[wasm_bindgen]
#[derive(Debug)]
pub enum SortDirection {
    Asc = "asc",
    Desc = "desc",
}

/// A struct passed to the JavaScript grid which is used by AG Grid to fetch the
/// requested data from the server.
#[wasm_bindgen]
#[derive(ToJsValueMacro)]
pub struct DataSource {
    #[wasm_bindgen(readonly, getter_with_clone, js_name = getRows)]
    pub get_rows: Function,
}

/// Builder for the [`DataSource`].
pub struct DataSourceBuilder {
    // Callback the grid calls that the user implements to fetch rows from the
    // server.
    get_rows: Closure<dyn FnMut(IGetRowsParams)>,
    // row_count is deprecated. Use GridOptions.infiniteInitialRowCount instead:
    // https://github.com/ag-grid/ag-grid/blob/7358e4286fd52946c4fe24bd26b5fbe7fd3b22d4/community-modules/core/src/ts/interfaces/iDatasource.ts#L7-L9
    //row_count: Option<u32>,
}

impl DataSourceBuilder {
    /// Start constructing a new `DataSourceBuilder` by providing a callback
    /// function which will receive [`GetRowsParams`]. This callback is
    /// called by AG Grid to request new rows from the server.
    pub fn new<F, Fut, T>(mut get_rows: F) -> Self
    where
        F: FnMut(GetRowsParams) -> Fut + 'static,
        Fut: Future<Output = Result<(Vec<T>, Option<u32>), Box<dyn std::error::Error>>> + 'static,
        T: ToJsValue,
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
                            log(&format!("Error calling get_rows callback: {e:?}"));
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

    /// Finalise construction of a [`DataSource`].
    pub fn build(self) -> DataSource {
        DataSource {
            get_rows: self.get_rows.into_js_value().unchecked_into(),
        }
    }
}

/// Allowed values for [`ColumnDef::sort`][crate::ColumnDef::sort] and related
/// methods.
#[derive(ToJsValueMacro)]
pub enum SortMethod {
    Asc,
    Desc,
    #[js_value(serialize_as = "null")]
    Null,
}

/// Allowed values for
/// [`GridOptions::row_model_type`][crate::GridOptions::row_model_type].
#[derive(ToJsValueMacro)]
pub enum RowModelType {
    Infinite,
    Viewport,
    ClientSide,
    ServerSide,
}

/// Allowed values for [`ColumnDef::filter`][crate::ColumnDef::filter].
#[derive(ToJsValueMacro)]
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
    #[js_value(serialize_as = "true")]
    True,
    /// Explicitly disable filtering.
    #[js_value(serialize_as = "false")]
    False,
    // TODO: Custom(FilterComponent)
}

/// Allowed values for
/// [`ColumnDef::lock_position`][crate::ColumnDef::lock_position].
#[derive(ToJsValueMacro)]
pub enum LockPosition {
    #[js_value(serialize_as = "true")]
    True,
    False,
    Left,
    Right,
}

/// Allowed values for
/// [`ColumnDef::pinned`][crate::ColumnDef::pinned] and
/// [`ColumnDef::initial_pinned`][crate::ColumnDef::initial_pinned].
#[derive(ToJsValueMacro)]
pub enum PinnedPosition {
    #[js_value(serialize_as = "true")]
    True,
    False,
    Left,
    Right,
}

/// Allowed values for
/// [`ColumnDef::cell_editor_popup_position`][crate::ColumnDef::cell_editor_popup_position].
#[derive(ToJsValueMacro)]
pub enum PopupPosition {
    Over,
    Under,
}

/// Allowed values for
/// [`ColumnDef::menu_tabs`][crate::ColumnDef::menu_tabs].
#[allow(clippy::enum_variant_names)]
#[derive(ToJsValueMacro)]
pub enum MenuTab {
    FilterMenuTab,
    GeneralMenuTab,
    ColumnsMenuTab,
}

pub(crate) enum OneOrMany<T>
where
    T: ToJsValue,
{
    One(T),
    Many(Vec<T>),
}

impl<T> ToJsValue for OneOrMany<T>
where
    T: ToJsValue,
{
    fn to_js_value(&self) -> JsValue {
        match self {
            Self::One(v) => v.to_js_value(),
            Self::Many(v) => v.to_js_value(),
        }
    }
}

impl<T> From<T> for OneOrMany<T>
where
    T: ToJsValue,
{
    fn from(v: T) -> Self {
        Self::One(v)
    }
}

impl<T> From<Vec<T>> for OneOrMany<T>
where
    T: Into<OneOrMany<T>> + ToJsValue,
{
    fn from(v: Vec<T>) -> Self {
        Self::Many(v)
    }
}
