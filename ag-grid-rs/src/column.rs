use ag_grid_derive::FieldSetter;
use serde::Serialize;
use serde_with::skip_serializing_none;
use wasm_bindgen::prelude::*;

use crate::Filter;

#[wasm_bindgen]
extern "C" {
    pub type ColumnApi;
}

#[skip_serializing_none]
#[derive(Serialize, FieldSetter)]
#[serde(rename_all = "camelCase")]
pub struct ColumnDef {
    // Base
    //
    /// The field of the row object to get the cell's data from. Deep references into a row object is
    /// supported via dot notation, i.e 'address.firstLine'.
    field: Option<String>,

    /// The unique ID to give the column. This is optional. If missing, the ID will default to the field. If both field
    /// and colId are missing, a unique ID will be generated. This ID is used to identify the column in the
    /// API for sorting, filtering etc.
    col_id: Option<String>,
    // TODO

    // Display
    // TODO

    // Editing
    // TODO

    // Events
    // TODO

    // Filter
    //
    /// Set wether the column is filterable.
    filter: Option<Filter>,

    /// Whether to displat a floating filter for this column.
    floating_filter: Option<bool>,
    // TODO

    // Header
    // TODO

    // Integrated Charts
    // TODO

    // Pinned
    // TODO

    // Pivoting
    // TODO

    // Rendering and Styling
    // TODO

    // Row Dragging
    // TODO

    // Row Grouping
    // TODO

    // Sort
    //
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
            .floating_filter(false);

        let expected = json!({
            "field": "make",
            "colId": "col_id",
            "sortable": true,
            "filter": "agDateColumnFilter",
            "floatingFilter": false,
        });

        assert_eq!(to_value(col).unwrap(), expected);
    }
}
