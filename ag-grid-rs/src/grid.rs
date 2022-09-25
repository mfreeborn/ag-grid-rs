//! Types pertaining to the `Grid` itself.

use ag_grid_core::convert::ToJsValue;
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

use crate::{column::ColumnApi, gridoptions::DataSource};

/// A handle to the underlying JavaScript grid.
pub struct Grid {
    // /// The [`GridOptions`] struct used to construct the grid.
    // pub grid_options: GridOptions<T>,
    /// A handle for the AG Grid [`Grid API`].
    ///
    /// [`Grid API`]: https://www.ag-grid.com/javascript-data-grid/grid-api/
    pub api: GridApi,
    /// A handle for the AG Grid [`Column API`].
    ///
    /// [`Column API`]: https://www.ag-grid.com/javascript-data-grid/column-api/
    pub column_api: ColumnApi,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = Grid)]
    pub(crate) type AgGrid;

    #[wasm_bindgen(js_name = GridOptions)]
    pub(crate) type AgGridOptions;

    #[wasm_bindgen(constructor, js_namespace = agGrid, js_class = "Grid")]
    pub(crate) fn new(eGridDiv: HtmlElement, gridOptions: JsValue) -> AgGrid;

    #[wasm_bindgen(method, getter)]
    pub(crate) fn gridOptions(this: &AgGrid) -> AgGridOptions;

    #[wasm_bindgen(method, getter)]
    pub(crate) fn api(this: &AgGridOptions) -> GridApi;

    #[wasm_bindgen(method, getter)]
    pub(crate) fn columnApi(this: &AgGridOptions) -> ColumnApi;
}

#[wasm_bindgen]
extern "C" {
    /// A handle for the AG Grid [`Grid API`].
    ///
    /// [`Grid API`]: https://www.ag-grid.com/javascript-data-grid/grid-api/
    pub type GridApi;

    #[wasm_bindgen(method)]
    fn exportDataAsCsv(this: &GridApi);

    #[wasm_bindgen(method)]
    fn setRowData(this: &GridApi, data: JsValue);

    #[wasm_bindgen(method)]
    fn setDatasource(this: &GridApi, data_source: DataSource);
}

impl GridApi {
    /// Download a CSV export of the grid's data.
    pub fn export_data_as_csv(&self) {
        Self::exportDataAsCsv(self)
    }

    /// Set the row data. Applicable when using
    /// [`RowModelType::ClientSide`][crate::gridoptions::RowModelType::ClientSide].
    pub fn set_row_data<T>(&self, row_data: Vec<T>)
    where
        T: ToJsValue,
    {
        Self::setRowData(self, row_data.to_js_value())
    }

    /// Set a new datasource. Applicable when using
    /// [`RowModelType::Infinite`][crate::gridoptions::RowModelType::Infinite].
    pub fn set_data_source(&self, data_source: DataSource) {
        Self::setDatasource(self, data_source)
    }
}
