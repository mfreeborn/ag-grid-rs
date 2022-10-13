//! Types pertaining to grid sorting.

use ag_grid_derive::FromInterface;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub(crate) type ISortModelItem;

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

/// Possible directions for which to sort data.
#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum SortDirection {
    Asc = "asc",
    Desc = "desc",
}
