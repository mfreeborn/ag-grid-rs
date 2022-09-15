use ag_grid_derive::FieldSetter;
use serde::Serialize;
use serde_with::skip_serializing_none;
use wasm_bindgen::prelude::*;

use crate::{Filter, LockPosition, MenuTab, OneOrMany, PinnedPosition, PopupPosition};

#[wasm_bindgen]
extern "C" {
    pub type ColumnApi;
}

#[skip_serializing_none]
#[derive(Serialize, FieldSetter)]
#[serde(rename_all = "camelCase")]
pub struct ColumnDef {
    // Base
    /// The field of the row object to get the cell's data from. Deep references
    /// into a row object is supported via dot notation, i.e
    /// 'address.firstLine'.
    field: Option<String>,

    /// The unique ID to give the column. This is optional. If missing, the ID
    /// will default to the field. If both field and colId are missing, a
    /// unique ID will be generated. This ID is used to identify the column in
    /// the API for sorting, filtering etc.
    col_id: Option<String>,

    //#[field_setter(skip)]
    /// A comma separated string or if using the `*_array` method, a vector of
    /// strings containing ColumnType keys which can be used as a template
    /// for a column. This helps to reduce duplication of properties when you
    /// have a lot of common column properties.
    type_: Option<OneOrMany<String>>,

    /// Set to `true` to display a disabled checkbox when row is not selectable
    /// and checkboxes are enabled.
    show_disabled_checkboxes: Option<bool>,
    // TODO

    // Display
    /// Set to `true` for this column to be hidden.
    hide: Option<bool>,

    /// Same as [`hide`], except only applied when creating a new column. Not
    /// applied when updating column definitions.
    initial_hide: Option<bool>,

    /// Set to `true` to block making column visible/hidden via the UI (API will
    /// still work).
    lock_visibile: Option<bool>,

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
    /// to 'Left'.
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
    /// with [`Column_def::auto_height`].
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
    // TODO

    // Spanning
    // TODO

    // Tooltips
    // TODO

    // Width
    // TODO

    // Groups
    // TODO

    // Groups: Header
    // TODO
}

impl ColumnDef {
    pub fn new() -> Self {
        Default::default()
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{json, to_value};

    use super::*;

    #[test]
    fn test_serialize_column_def() {
        let col = ColumnDef::new()
            .field("make")
            .col_id("col_id")
            .sortable(true)
            .filter(Filter::AgDateColumnFilter)
            .floating_filter(false)
            .type_("hi".to_string())
            .type_("hi")
            .type_array(vec!["hi".to_string()])
            .type_array(vec!["hi", "there"]);

        let expected = json!({
            "field": "make",
            "colId": "col_id",
            "sortable": true,
            "filter": "agDateColumnFilter",
            "floatingFilter": false,
            "type": ["hi", "there"]
        });

        assert_eq!(to_value(col).unwrap(), expected);
    }
}
