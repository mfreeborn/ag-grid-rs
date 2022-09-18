use ag_grid_derive::{FieldSetter, ToJsValue};
use web_sys::HtmlElement;

use crate::{
    column::ColumnDef, traits::ToJsValue as _, AgGrid, DataSource, Grid, RowData, RowModelType,
};

/// An instance of an AG Grid [`GridOptions`].
///
/// With this struct, users can specify the initial options for their grid,
/// before calling the [`build()`] method to receive an instance of [`Grid`].
/// The various options are fully customisable using the builder pattern, so you
/// only need to specify what you need. The options mirror those used in the AG
/// Grid library.
///
/// [`GridOptions`]: https://www.ag-grid.com/javascript-data-grid/grid-options/
#[derive(FieldSetter, ToJsValue)]
pub struct GridOptions {
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
    row_data: Option<Vec<RowData>>,

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

impl GridOptions {
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
            grid_options: self,
            api: js_grid.gridOptions().api(),
            column_api: js_grid.gridOptions().columnApi(),
        }
    }
}
