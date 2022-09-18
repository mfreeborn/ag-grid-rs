use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

use crate::{traits::ToJsValue, ColumnApi, DataSource, GridOptions, RowData};

/// An instance of an AG Grid.
pub struct Grid {
    /// The `GridOptions` struct used to construct the grid.
    pub grid_options: GridOptions,
    /// An accessor for the [`Grid API`].
    ///
    /// [`Grid API`]: https://www.ag-grid.com/javascript-data-grid/grid-api/
    pub api: GridApi,
    /// An accessor for the [`Coluumn API`].
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
    pub type GridApi;

    #[wasm_bindgen(method)]
    fn exportDataAsCsv(this: &GridApi);

    #[wasm_bindgen(method)]
    fn setRowData(this: &GridApi, data: JsValue);

    #[wasm_bindgen(method)]
    fn setDatasource(this: &GridApi, data_source: DataSource);
}

impl GridApi {
    pub fn export_data_as_csv(&self) {
        Self::exportDataAsCsv(self)
    }

    pub fn set_row_data(&self, row_data: Vec<RowData>) {
        Self::setRowData(self, row_data.to_js_value())
    }

    pub fn set_data_source(&self, data_source: DataSource) {
        Self::setDatasource(self, data_source)
    }
}
