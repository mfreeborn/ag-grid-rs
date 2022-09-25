//! A collection of parameter types passed to callback functions.

use std::collections::HashMap;

use ag_grid_core::imports::ObjectExt;
use ag_grid_derive::FromInterface;
use js_sys::{Function, Object};
use wasm_bindgen::{prelude::*, JsCast};

use crate::{
    filter::{CombinedFilterModel, FilterModel, FilterModelType},
    sort::{ISortModelItem, SortModelItem},
};

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
    pub(crate) fn success_callback(this: &IGetRowsParams) -> Function;

    #[wasm_bindgen(method, getter, js_name = failCallback)]
    pub(crate) fn fail_callback(this: &IGetRowsParams) -> Function;
}

/// Parameters passed to the callback function in
/// [`DataSourceBuilder::new`][`crate::gridoptions::DataSourceBuilder`].
#[derive(Debug)]
pub struct GetRowsParams {
    /// The first row index to get.
    pub start_row: u32,
    /// The first row index to *not* get.
    pub end_row: u32,
    /// A vector of `[SortModelItem]` describing how the data is expected to be
    /// sorted.
    pub sort_model: Vec<SortModelItem>,
    /// Details of how to filter the requested data.
    pub filter_model: HashMap<String, FilterModelType>,
}

impl From<&IGetRowsParams> for GetRowsParams {
    fn from(i: &IGetRowsParams) -> Self {
        Self {
            start_row: i.start_row(),
            end_row: i.end_row(),
            sort_model: i.sort_model().iter().map(SortModelItem::from).collect(),
            filter_model: {
                let filter_object = i.filter_model().unchecked_into::<ObjectExt>();
                let mut filters = Vec::new();

                for (col, filter) in filter_object.entries() {
                    let filter = filter.unchecked_into::<ObjectExt>();

                    let filter = if filter.has_own_property(&"operator".into()) {
                        let filter = CombinedFilterModel::from_object(&filter);
                        FilterModelType::Combined(filter)
                    } else {
                        let filter = FilterModel::from_object(&filter);
                        FilterModelType::Single(filter)
                    };

                    let filter = (col, filter);
                    filters.push(filter);
                }

                filters.into_iter().collect()
            },
        }
    }
}
