//! Types pertaining to the grid columns.

use ag_grid_derive::{FieldSetter, ToJsValue};
use wasm_bindgen::prelude::*;

pub use crate::shared::SortMethod;
use crate::{callbacks::IHeaderValueGetterParams, types::OneOrMany};

#[wasm_bindgen]
extern "C" {
    /// A handle for the AG Grid [`Column API`].
    ///
    /// [`Column API`]: https://www.ag-grid.com/javascript-data-grid/column-api/
    pub type ColumnApi;
}

/// A customisable struct for defining a column.
#[derive(FieldSetter, ToJsValue)]
#[js_value(skip_serializing_none)]
pub struct ColumnDef {
    // Base
    field: Option<String>,

    /// The unique ID to give the column. This is optional. If missing, the ID
    /// will default to the field. If both field and colId are missing, a
    /// unique ID will be generated. This ID is used to identify the column in
    /// the API for sorting, filtering etc.
    col_id: Option<String>,

    /// A comma separated string or if using the [`ColumnDef::type_array`]
    /// method, a vector of strings containing ColumnType keys which can be
    /// used as a template for a column. This helps to reduce duplication of
    /// properties when you have a lot of common column properties.
    type_: Option<OneOrMany<String>>,

    // TODO: support callback function
    /// A function or expression that gets the value to be displayed from your
    /// data.
    value_getter: Option<String>,

    // TODO: support callback function
    /// A function or an expression to format a value. Not used for CSV export
    /// or copy to clipboard; only for UI cell rendering.
    value_formatter: Option<String>,

    /// Provide a reference data map to be used to map column values to their
    /// respective value from the map.
    //ref_data: Option<HashMap<String, String>>,

    /// Set to `true` to display a disabled checkbox when row is not selectable
    /// and checkboxes are enabled.
    show_disabled_checkboxes: Option<bool>,
    // TODO

    // Display
    /// Set to `true` for this column to be hidden.
    hide: Option<bool>,

    /// Same as [`ColumnDef::hide`], except only applied when creating a new
    /// column. Not applied when updating column definitions.
    initial_hide: Option<bool>,

    /// Set to `true` to block making column visible/hidden via the UI (API will
    /// still work).
    lock_visible: Option<bool>,

    /// Lock a column to position to `Left` or `Right` to always have this
    /// column displayed in that position. `True` is treated as `Left`.
    lock_position: Option<LockPosition>,

    /// Set to `true` if you do not want this column to be movable via dragging.
    suppress_movable: Option<bool>,

    // Editing
    /// Set to `true` if this column is editable.
    editable: Option<bool>, // TODO: add support for a callback function

    /// Set to `true` to have the cell editor appear in a popup.
    cell_editor_popup: Option<bool>,

    /// Set the position for the popup cell editor. Possible values are `Over`,
    /// whereby the popup will be positioned over the cell, or `Under`, whereby
    /// the popup will be positioned below the cell leaving the cell value
    /// visible.
    cell_editor_popup_position: Option<PopupPosition>,

    /// Set to `true` to have cells under this column enter edit mode after
    /// single click.
    single_click_edit: Option<bool>,
    // TODO

    // Events
    // TODO

    // Filter
    /// Set whether the column is filterable, or use one of the provided
    /// filters.
    filter: Option<Filter>,

    /// Whether to display a floating filter for this column.
    floating_filter: Option<bool>,
    // TODO

    // Header
    /// The name to render in the column header. If not specified and field is
    /// specified, the field name will be used as the header name.
    header_name: Option<String>,

    /// Get the value for display in the header.
    header_value_getter: Option<Closure<dyn FnMut(IHeaderValueGetterParams) -> String>>,

    /// Tooltip for the column header.
    header_tooltip: Option<String>,

    /// CSS class to use for the header cell. Can be a string or, if using the
    /// `header_class_array` method, a vector of strings.
    header_class: Option<OneOrMany<String>>,

    /// Set to `true` to wrap long header names onto the next line.
    wrap_header_text: Option<bool>,

    /// Set to `true` to enable the header row to automatically adjust its
    /// height to accommodate the size of the header cell.
    auto_header_height: Option<bool>,

    /// Select which menu tabs are present, and in what order they are shown.
    menu_tabs: Option<Vec<MenuTab>>,

    /// Set to `true` to disable showing the menu for this column header.
    suppress_menu: Option<bool>,

    /// If `true`, a 'select all' checkbox will be put into the header.
    header_checkbox_selection: Option<bool>, // TODO: add support for a callback function

    /// If `true`, the header checkbox selection will only select filtered
    /// items.
    header_checkbox_selection_filtered_only: Option<bool>,
    // TODO

    // Integrated Charts
    // All options are enterprise-only

    // Pinned
    /// Pin a column to one side: right or left. A value of `True` is converted
    /// to `Left`.
    pinned: Option<PinnedPosition>,

    /// Same as [`ColumnDef::pinned`], except only applied when creating a new
    /// column. Not applied when updating column definitions.
    initial_pinned: Option<PinnedPosition>,

    /// Set to `true` to block the user pinning the column, the column can only
    /// be pinned via definitions or API.
    lock_pinned: Option<bool>,

    // Pivoting
    // All options are enterprise-only

    // Rendering and Styling
    /// Set to `true` to have the grid calculate the height of a row based on
    /// contents of this column.
    auto_height: Option<bool>,

    /// Set to `true` to have the text wrap inside the cell - typically used
    /// with [`ColumnDef::auto_height`].
    wrap_text: Option<bool>,

    /// Set to `true` to flash a cell when it's refreshed.
    enable_cell_change_flash: Option<bool>,

    /// Set to `true` to prevent this column from flashing on changes. Only
    /// applicable if cell flashing is turned on for the grid.
    suppress_cell_flash: Option<bool>,
    // TODO

    // Row Dragging
    /// Set to `true` to allow row dragging.
    row_drag: Option<bool>, // TODO: add support for callback function

    /// Set to `true` to allow dragging for native drag and drop.
    dnd_source: Option<bool>, // TODO: add support for callback function
    // TODO

    // Row Grouping
    // All options and enterprise-only

    // Sort
    /// Set wether the column is sortable.
    sortable: Option<bool>,

    /// Set the default sorting method.
    sort: Option<SortMethod>,

    /// The same as [`ColumnDef::sort`], except only applied when creating a new
    /// column. Not applied when updating column definitions.
    initial_sort: Option<SortMethod>,

    /// If sorting more than one column by default, specifies order in which the
    /// sorting should be applied.
    sort_index: Option<Option<u32>>,

    /// Vector defining the order in which sorting occurs (if sorting is
    /// enabled). Expects a vector of any permutation of the [`SortMethod`]
    /// variants.
    sorting_order: Option<Vec<SortMethod>>,

    /// Set to `true` if you want the unsorted icon to be shown when no sort is
    /// applied to this column.
    #[js_value(rename = "unSortIcon")]
    unsort_icon: Option<bool>,

    // Spanning
    //
    // TODO: support callback function
    /// Set the span of the column.
    col_span: Option<u32>,

    // TODO: support callback function
    /// Set the span of the row.
    row_span: Option<u32>,

    // Tooltips
    /// The field of the tooltip to apply to the cell.
    tooltip_field: Option<String>, // TODO

    // Width
    /// Initial width in pixels for the cell.
    width: Option<u32>,

    /// The same as [`ColumnDef::width`], except only applied when creating a
    /// new column. Not applied when updating column definitions.
    initial_width: Option<u32>,

    /// Minimum width in pixels for the cell.
    min_width: Option<u32>,

    /// Maxmum width in pixels for the cell.
    max_width: Option<u32>,

    /// Used instead of width when the goal is to fill the remaining empty space
    /// of the grid.
    flex: Option<u32>,

    /// The same as [`ColumnDef::flex`], except only applied when creating a new
    /// column. Not applied when updating column definitions.
    initial_flex: Option<u32>,

    /// Set to `true` to allow this column to be resized.
    resizable: Option<bool>,

    /// Set to `true` if you want this column's width to be fixed during 'size
    /// to fit' operations.
    suppress_size_to_fit: Option<bool>,

    /// Set to `true` if you do not want this column to be auto-resizable by
    /// double clicking it's edge.
    suppress_auto_size: Option<bool>,
}

impl ColumnDef {
    /// Create a new column definition, specifying the field of the row object
    /// to get the cell's data from. Deep references into a row object is
    /// supported via dot notation, i.e 'address.firstLine'.
    pub fn new() -> Self {
        Default::default()
    }
}

/// Allowed values for [`ColumnDef::filter`][crate::ColumnDef::filter].
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
    #[js_value(serialize_as = "true")]
    True,
    /// Explicitly disable filtering.
    #[js_value(serialize_as = "false")]
    False,
    // TODO: Custom(FilterComponent)
}

/// Allowed values for
/// [`ColumnDef::lock_position`][crate::ColumnDef::lock_position].
#[derive(ToJsValue)]
pub enum LockPosition {
    #[js_value(serialize_as = "true")]
    True,
    #[js_value(serialize_as = "false")]
    False,
    Left,
    Right,
}

/// Allowed values for
/// [`ColumnDef::pinned`][crate::ColumnDef::pinned] and
/// [`ColumnDef::initial_pinned`][crate::ColumnDef::initial_pinned].
#[derive(ToJsValue)]
pub enum PinnedPosition {
    #[js_value(serialize_as = "true")]
    True,
    #[js_value(serialize_as = "false")]
    False,
    Left,
    Right,
}

/// Allowed values for
/// [`ColumnDef::cell_editor_popup_position`][crate::ColumnDef::cell_editor_popup_position].
#[derive(ToJsValue)]
pub enum PopupPosition {
    Over,
    Under,
}

/// Allowed values for
/// [`ColumnDef::menu_tabs`][crate::ColumnDef::menu_tabs].
#[allow(clippy::enum_variant_names)]
#[derive(ToJsValue)]
pub enum MenuTab {
    FilterMenuTab,
    GeneralMenuTab,
    ColumnsMenuTab,
}
