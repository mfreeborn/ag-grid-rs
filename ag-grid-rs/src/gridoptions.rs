//! Types pertaining to defining and constructing a `Grid`.

use std::{collections::HashMap, future::Future};

use ag_grid_core::imports::log;
use ag_grid_derive::FieldSetter;
use js_sys::Function;
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlElement;

pub use crate::shared::SortMethod;
use crate::{
    callbacks::{GetRowsParams, IGetRowsParams},
    column::ColumnDef,
    convert::ToJsValue,
    grid::AgGrid,
    types::OneOrMany,
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
    // TODO

    // Clipboard
    // TODO

    // Column Definitions
    /// Set the column definitions. Fields set here take precedence over those
    /// set in `default_col_def`.
    column_defs: Option<Vec<ColumnDef>>,
    /// Set the default column definition. Fields set here have lower precedence
    /// than fields set on a per-column basis in `column_defs`.
    default_col_def: Option<ColumnDef>,
    // default_col_group_def
    // column_types
    /// Keeps the order of Columns maintained after new Column Definitions are
    /// updated.
    maintain_column_order: Option<bool>,

    /// If `true`, then dots in field names (e.g. 'address.firstLine') are not
    /// treated as deep references. Allows you to use dots in your field name if
    /// you prefer.
    suppress_field_dot_notation: Option<bool>,

    // Column Headers
    /// The height in pixels for the row containing the column label header. If
    /// not specified, it uses the theme value of `header-height`.
    header_height: Option<u32>,

    /// The height in pixels for the rows containing header column groups. If
    /// not specified, it uses [`GridOptions::header_height`].
    group_header_height: Option<u32>,

    /// The height in pixels for the row containing the floating filters. If not
    /// specified, it uses the theme value of `header-height`.
    floating_filters_height: Option<u32>,

    // Column Moving
    /// Set to `true` to suppress column moving, i.e. to make the columns fixed
    /// position.
    suppress_movable_columns: Option<bool>,

    /// If `true`, the `ag-column-moving` class is not added to the grid while
    /// columns are moving. In the default themes, this results in no animation
    /// when moving columns.
    suppress_column_move_animation: Option<bool>,

    /// If `true`, when you drag a column out of the grid (e.g. to the group
    /// zone) the column is not hidden.
    suppress_drag_leave_hides_columns: Option<bool>,

    /// If `true`, when you drag a column into a row group panel the column is
    /// not hidden.
    suppress_row_group_hides_columns: Option<bool>,

    // Column Sizing
    /// Set to 'Shift' to have shift-resize as the default resize operation
    /// (same as user holding down `Shift` while resizing).
    col_resize_default: Option<ResizeMethod>,

    /// Suppresses auto-sizing columns for columns. In other words, double
    /// clicking a column's header's edge will not auto-size.
    suppress_auto_size: Option<bool>,

    /// Number of pixels to add to a column width after the auto-sizing
    /// calculation. Set this if you want to add extra room to accommodate (for
    /// example) sort icons, or some other dynamic nature of the header.
    auto_size_padding: Option<u32>,

    /// Set this to `true` to skip the `header_name` when `auto_size` is called
    /// by default.
    skip_header_on_auto_size: Option<bool>,

    // Components
    // TODO

    // Editing
    /// Set to 'FullRow' to enable Full Row Editing. Otherwise leave blank to
    /// edit one cell at a time.
    edit_type: Option<EditType>,

    /// Set to `true` to enable Single Click Editing for cells, to start editing
    /// with a single click.
    single_click_edit: Option<bool>,

    /// Set to `true` so that neither single nor double click starts editing.
    suppress_click_edit: Option<bool>,

    /// Set to `true` to stop cell editing when grid loses focus. The default is
    /// that the grid stays editing until focus goes onto another cell.
    stop_editing_when_cells_lose_focus: Option<bool>,

    /// Set to `true` along with [`GridOptions::enter_moves_down_after_edit`] to
    /// have Excel-style behaviour for the `Enter` key, i.e. pressing the
    /// `Enter` key will move down to the cell beneath.
    enter_moves_down: Option<bool>,

    /// Set to `true` along with [`GridOptions::enter_moves_down`] to have
    /// Excel-style behaviour for the `Enter` key, i.e. pressing the `Enter` key
    /// will move down to the cell beneath.
    enter_moves_down_after_edit: Option<bool>,

    /// Set to `true` to enable Undo / Redo while editing.
    undo_redo_cell_editing: Option<bool>,

    /// Set the size of the undo / redo stack.
    undo_redo_cell_editing_limit: Option<u32>,

    /// Set to `true` to stop the grid updating data after and edit. When this
    /// is set, it is intended the application will update the data, e.g. in an
    /// external immutable store, and then pass the new dataset to the grid.
    read_only_edit: Option<bool>,

    // Export
    /// Prevent the user from exporting the grid to CSV.
    suppress_csv_export: Option<bool>,

    /// Prevent the user from exporting the grid to Excel.
    suppress_excel_export: Option<bool>,
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
    /// Provide a context object that is provided to different callbacks the
    /// grid uses. Used for passing additional information to the callbacks by
    /// your application.
    context: Option<HashMap<String, String>>,

    /// Change this value to set the tabIndex order of the Grid within your
    /// application.
    tab_index: Option<u32>,

    /// The number of rows rendered outside the viewable area the grid renders.
    /// Having a buffer means the grid will have rows ready to show as the user
    /// slowly scrolls vertically.
    row_buffer: Option<u32>,

    /// Set to `true` to enable debug information from the grid and related
    /// components. Will result in additional logging being output, but very
    /// useful when investigating problems.
    debug: Option<bool>,

    // Overlays
    // TODO

    // Pagination
    /// Set whether pagination is enabled.
    pagination: Option<bool>,

    /// How many rows to load per page. If
    /// [`GridOptions::pagination_auto_page_size`] is specified, this property
    /// is ignored.
    pagination_page_size: Option<u32>,

    /// Set to `true` so that the number of rows to load per page is
    /// automatically adjusted by the grid so each page shows enough rows to
    /// just fill the area designated for the grid. If `false`,
    /// [#GridOption::pagination_page_size`] is used.
    pagination_auto_page_size: Option<bool>,

    /// Set to `true` to have pages split children of groups when using Row
    /// Grouping or detail rows with Master Detail.
    paginate_child_rows: Option<bool>,

    /// If `true`, the default grid controls for navigation are hidden. This is
    /// useful if `pagination=true` and you want to provide your own pagination
    /// controls. Otherwise, when `pagination=true` the grid automatically shows
    /// the necessary controls at the bottom so that the user can navigate
    /// through the different pages.
    suppress_pagination_panel: Option<bool>,

    // Pivot and Aggregation
    // TODO

    // Rendering
    /// Set to `true` to enable Row Animation.
    animate_rows: Option<bool>,

    /// Set to `true` to have cells flash after data changes.
    enable_cell_change_flash: Option<bool>,

    /// To be used in combination with
    /// [`GridOptions::enable_cell_change_flash`], this configuration
    /// will set the delay in milliseconds of how long a cell should remain in
    /// its "flashed" state.
    cell_flash_delay: Option<u32>,

    /// To be used in combination with
    /// [`GridOptions::enable_cell_change_flash`], this configuration
    /// will set the delay in milliseconds of how long the "flashed" state
    /// animation takes to fade away after the timer set by
    /// [`GridOptions::cell_flash_delay`] has completed.
    cell_fade_delay: Option<u32>,

    /// Set to `true` to have cells flash after data changes even when the
    /// change is due to filtering.
    allow_show_change_after_filter: Option<bool>,

    /// Switch between layout options.
    dom_layout: Option<DomLayout>,

    /// When `true`, the order of rows and columns in the DOM are consistent
    /// with what is on screen.
    ensure_dom_order: Option<bool>,

    /// Set to `true` to operate the grid in RTL (Right to Left) mode.
    enable_rtl: Option<bool>,

    /// Set to `true` so that the grid doesn't virtualise the columns. For
    /// example, if you have 100 columns, but only 10 visible due to scrolling,
    /// all 100 will always be rendered.
    suppress_column_virtualisation: Option<bool>,

    /// Set to `true` so that the grid doesn't virtualise the rows. For example,
    /// if you have 100 rows, but only 10 visible due to scrolling, all 100 will
    /// always be rendered.
    suppress_row_virtualisation: Option<bool>,

    /// By default the grid has a limit of rendering a maximum of 500 rows at
    /// once (remember the grid only renders rows you can see, so unless your
    /// display shows more than 500 rows without vertically scrolling this will
    /// never be an issue). This is only relevant if you are manually setting
    /// [`GridOptions::row_buffer`] to a high value (rendering more rows than
    /// can be seen) or if your grid height is able to display more than 500
    /// rows at once.
    suppress_max_rendered_row_restriction: Option<bool>,

    // Row Drag and Drop
    /// Set to `true` to enable Managed Row Dragging.
    row_drag_managed: Option<bool>,

    /// Set to `true` to enable clicking and dragging anywhere on the row
    /// without the need for a drag handle.
    row_drag_entire_row: Option<bool>,

    /// Set to `true` to enable dragging multiple rows at the same time.
    row_drag_multi_row: Option<bool>,

    /// Set to `true` to suppress row dragging.
    suppress_row_drag: Option<bool>,

    /// Set to `true` to suppress moving rows while dragging the row drag
    /// waffle. This option highlights the position where the row will be
    /// placed and it will only move the row on mouse up.
    suppress_move_when_row_dragging: Option<bool>,

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
    /// Set to `true` to always show the horizontal scrollbar.
    always_show_horizontal_scroll: Option<bool>,

    /// Set to `true` to always show the vertical scrollbar.
    always_show_vertical_scroll: Option<bool>,

    /// Set to `true` to debounce the vertical scrollbar. Can provide smoother
    /// scrolling on slow machines.
    debounce_vertical_scrollbar: Option<bool>,

    /// Set to `true` to never show the horizontal scroll. This is useful if the
    /// grid is aligned with another grid and will scroll when the other grid
    /// scrolls. (Should not be used in combination with
    /// [`GridOptions::always_show_horizontal_scroll`].)
    suppress_horizontal_scroll: Option<bool>,

    /// When `true`, the grid will not scroll to the top when new row data is
    /// provided. Use this if you don't want the default behaviour of scrolling
    /// to the top every time you load new data.
    suppress_scroll_on_new_data: Option<bool>,

    /// When `true`, the grid will not allow mousewheel/touchpad scroll when
    /// popup elements are present.
    suppress_scroll_when_popups_are_open: Option<bool>,

    /// When `true`, the grid will not use animation frames when drawing rows
    /// while scrolling. Use this if the grid is working fast enough that you
    /// don't need animation frames and you don't want the grid to flicker.
    suppress_animation_frame: Option<bool>,

    /// When `true`, middle clicks will result in click events for cells and
    /// rows. Otherwise the browser will use middle click to scroll the grid.
    /// Note: Not all browsers fire click events with the middle button. Most
    /// will fire only mousedown and mouseup events, which can be used to focus
    /// a cell, but will not work to call the onCellClicked function.
    suppress_middle_click_scrolls: Option<bool>,

    /// When `true`, mouse wheel events will be passed to the browser. Useful if
    /// your grid has no vertical scrolls and you want the mouse to scroll the
    /// browser page.
    suppress_prevent_default_on_mouse_wheel: Option<bool>,

    /// Tell the grid how wide in pixels the scrollbar is, which is used in grid
    /// width calculations. Set only if using non-standard browser-provided
    /// scrollbars, so the grid can use the non-standard size in its
    /// calculations.
    scrollbar_width: Option<u32>,

    // Selection
    /// Type of row selection.
    row_selection: Option<RowSelection>,

    /// Set to `true` to allow multiple rows to be selected using single click.
    row_multi_select_with_click: Option<bool>,

    // is_row_selectable
    /// If `true`, rows will not be deselected if you hold down `Ctrl` and click
    /// the row or press `Space`.
    suppress_row_deselection: Option<bool>,

    /// If `true`, row selection won't happen when rows are clicked. Use when
    /// you only want checkbox selection.
    suppress_row_click_selection: Option<bool>,

    /// If `true`, cells won't be focusable. This means keyboard navigation will
    /// be disabled for grid cells, but remain enabled in other elements of the
    /// grid such as column headers, floating filters, tool panels.
    suppress_cell_focus: Option<bool>,

    /// Set to `true` to be able to select the text within cells. Note: When
    /// this is set to true, the clipboard service is disabled.
    enable_cell_text_selection: Option<bool>,

    // Sorting
    /// Vector defining the order in which sorting occurs (if sorting is
    /// enabled).
    sorting_order: Option<Vec<SortMethod>>,

    /// Set to `true` to specify that the sort should take accented characters
    /// into account. If this feature is turned on the sort will be slower.
    accented_sort: Option<bool>,

    /// Set to `true` to show the 'no sort' icon.
    #[js_value(rename = "unSortIcon")]
    unsort_icon: Option<bool>,

    /// Set to `true` to suppress multi-sort when the user shift-clicks a column
    /// header.
    suppress_multi_sort: Option<bool>,

    /// Set to `true` to always multi-sort when the user clicks a column header,
    /// regardless of key presses.
    always_multi_sort: Option<bool>,

    /// Set to 'Ctrl' to have multi sorting work using the `Ctrl` (or `Command
    /// ⌘` for Mac) key.
    multi_sort_key: Option<MultiSortKey>,

    /// Set to `true` to suppress sorting of un-sorted data to match original
    /// row data.
    suppress_maintain_unsorted_order: Option<bool>,

    /// When enabled, sorts only the rows added/updated by a transaction.
    delta_sort: Option<bool>,

    // Styling
    /// Default row height in pixels.
    row_height: Option<u32>,

    /// CSS class(es) for all rows. Provide either a string (class name) or
    /// vector of strings (vector of class names).
    row_class: Option<OneOrMany<String>>,

    /// Set to `true` to not highlight rows by adding the `ag-row-hover` CSS
    /// class.
    suppress_row_hover_highlight: Option<bool>,

    /// Uses CSS `top` instead of CSS `transform` for positioning rows. Useful
    /// if the transform function is causing issues such as used in `row
    /// spanning`.
    suppress_row_transform: Option<bool>,

    /// Set to `true` to highlight columns by adding the `ag-column-hover` CSS
    /// class.
    column_hover_highlight: Option<bool>,

    // Tooltips
    /// Set to `true` to use the browser's default tooltip instead of using the
    /// grid's Tooltip Component.
    enable_browser_tooltips: Option<bool>,

    /// The delay in milliseconds that it takes for tooltips to show up once an
    /// element is hovered over. Note: This property does not work if
    /// [`GridOptions::enable_browser_tooltips`] is `true`.
    tooltip_show_delay: Option<u32>,

    /// The delay in milliseconds that it takes for tooltips to hide once they
    /// have been displayed. Note: This property does not work if
    /// [`GridOptions::enable_browser_tooltips`] is `true`.
    tooltip_hide_delay: Option<u32>,

    /// Set to `true` to have tooltips follow the cursor once they are
    /// displayed.
    tooltip_mouse_track: Option<bool>,
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

/// Allowed values for [`GridOptions::multi_sort_key`].
#[derive(ToJsValueMacro)]
pub enum MultiSortKey {
    Ctrl,
}

/// Allowed values for [`GridOptions::dom_layout`].
#[derive(ToJsValueMacro)]
pub enum DomLayout {
    Normal,
    Print,
    AutoHeight,
}

/// Allowed values for [`GridOptions::edit_type`].
#[derive(ToJsValueMacro)]
pub enum EditType {
    FullRow,
}

/// Allowed values for [`GridOptions::col_resize_default`].
#[derive(ToJsValueMacro)]
pub enum ResizeMethod {
    Shift,
}

/// Allowed values for [`GridOptions::row_selection`].
#[derive(ToJsValueMacro)]
pub enum RowSelection {
    Single,
    Multiple,
}

/// Allowed values for [`GridOptions::row_model_type`].
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
    // row_count: Option<u32>,
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
