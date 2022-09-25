//! Types pertaining to defining and constructing a `Grid`.

use std::future::Future;

use ag_grid_core::imports::log;
use ag_grid_derive::FieldSetter;
use js_sys::Function;
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlElement;

use crate::{
    callbacks::{GetRowsParams, IGetRowsParams},
    column::ColumnDef,
    convert::ToJsValue,
    grid::AgGrid,
    Grid, ToJsValue as ToJsValueMacro,
};
/// An instance of an AG Grid [`GridOptions`].
///
/// With this struct, users can specify the initial options for their grid,
/// before calling the [`GridOptions::build()`] method to receive an instance of
/// [`Grid`]. The various options are fully customisable using the builder
/// pattern, so you only need to specify what you need. The options mirror those
/// used in the AG Grid library.
///
/// [`GridOptions`]: https://www.ag-grid.com/javascript-data-grid/grid-options/
#[derive(FieldSetter, ToJsValueMacro)]
#[js_value(skip_serializing_none)]
pub struct GridOptions<T>
where
    T: ToJsValue,
{
    // Accessories
    // All options are enterprise-only

    // Clipboard
    // All options are enterprise-only

    // Column Definitions
    /// Set the column definitions. Fields set here take precedence over those
    /// set in `default_col_def`.
    column_defs: Option<Vec<ColumnDef>>,
    /// Set the default column definition. Fields set here have lower precedence
    /// than fields set on a per-column basis in `column_defs`.
    default_col_def: Option<ColumnDef>,
    // default_col_group_def
    // column_types
    // maintain_column_order
    // suppress_dot_field_notation

    // Column Headers
    // TODO

    // Column Moving
    // TODO

    // Column Sizing
    // TODO

    // Components
    // TODO

    // Editing
    // TODO

    // Export
    // TODO

    // Filtering
    // TODO

    // Integrated Charts
    // TODO

    // Keyboard Navigation
    // TODO

    // Loading Cell Renderers
    // TODO

    // Localisation
    // TODO

    // Master Detail
    // TODO

    // Miscellaneous
    // TODO

    // Overlays
    // TODO

    // Pagination
    /// Set whether pagination is enabled.
    pagination: Option<bool>,
    // TODO

    // Pivot and Aggregation
    // TODO

    // Rendering
    // TODO

    // Row Drag and Drop
    // TODO

    // Row Full Width
    // TODO

    // Row Pinning
    // TODO

    // RowModel
    /// Sets the row model type.
    row_model_type: Option<RowModelType>,
    // get_row_id

    // RowModel: Client Side
    /// Set the row data.
    row_data: Option<Vec<T>>,

    // RowModel: Infinite
    datasource: Option<DataSource>,
    /// How many extra blank rows to display to the user at the end of the
    /// dataset, which sets the vertical scroll and then allows the grid to
    /// request viewing more rows of data.
    cache_overflow_size: Option<u32>,

    /// How many requests to hit the server with concurrently. If the max is
    /// reached, requests are queued. Set to `-1` for no maximum restriction on
    /// requests.
    max_concurrent_datasource_requests: Option<i32>,

    /// How many rows for each block in the store, i.e. how many rows returned
    /// from the server at a time.
    cache_block_size: Option<u32>,

    /// How many blocks to keep in the store. Default is no limit, so every
    /// requested block is kept. Use this if you have memory concerns, and
    /// blocks that were least recently viewed will be purged when the limit is
    /// hit. The grid will additionally make sure it has all the blocks needed
    /// to display what is currently visible, in case this property is set to a
    /// low value.
    max_blocks_in_cache: Option<u32>,

    /// How many extra blank rows to display to the user at the end of the
    /// dataset, which sets the vertical scroll and then allows the grid to
    /// request viewing more rows of data.
    infinite_initial_row_count: Option<u32>,
    // TODO

    // RowModel: Server Side
    // TODO

    // RowModel: Viewport
    // TODO

    // Scrolling
    // TODO

    // Selection
    // TODO

    // Sorting
    // TODO

    // Styling
    // TODO

    // Tooltips
    // TODO
}

impl<T> GridOptions<T>
where
    T: ToJsValue,
{
    pub fn new() -> Self {
        Default::default()
    }

    /// A finaliser method for the [`GridOptions`] struct. This method
    /// constructs the underlying JavaScript grid and returns a handle,
    /// [`Grid`], which provides access to the grid APIs.
    pub fn build(self, div: HtmlElement) -> Grid {
        let grid_options = self.to_js_value();

        let js_grid = AgGrid::new(div, grid_options);

        Grid {
            api: js_grid.gridOptions().api(),
            column_api: js_grid.gridOptions().columnApi(),
        }
    }
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
