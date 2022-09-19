//! Rust bindings for the [`AG Grid`] JavaScript library.
//!
//! With this crate, one is able to use the AG Grid datatable library within a
//! Wasm context in Rust.
//!
//! A simple example demonstrating server-side data fetching using `yew` and
//! related dependencies is as follows:
//!
//! ```rust
//! use ag_grid_rs::{ColumnDef, DataSourceBuilder, GridOptions, RowModelType,
//! ToJsValue};
//! use gloo_net::http::Request;
//! use serde::Deserialize;
//! use wasm_bindgen::JsCast;
//! use web_sys::HtmlElement;
//! use yew::prelude::*;
//!
//! #[function_component(About)]
//! pub fn about() -> Html {
//!     use_effect_with_deps(
//!         |_| {
//!             // Get the element to which you want to attach the grid
//!             let grid_div = get_element_by_id("grid-div");
//!
//!             // Define your columns
//!             let field_names = vec!["athlete", "age", "country", "year"];
//!             let cols = field_names
//!                 .iter()
//!                 .map(|name| ColumnDef::new(name).sortable(true))
//!                 .collect();
//!
//!             // Create your datasource, including a closure that will retunr rows from the
//!             // server
//!             let data_source = DataSourceBuilder::new(|params| async move {
//!                 // `params` contains information from AG Grid about which rows to get, how to
//!                 // sort the data, etc
//!                 let data_url = "https://www.ag-grid.com/example-assets/olympic-winners.json";
//!                 let rows = gloo_net::http::Request::get(data_url)
//!                     .send()
//!                     .await?
//!                     .json::<Vec<JsonData>>()
//!                     .await?;
//!
//!                 Ok((rows, None))
//!             })
//!             .build();
//!
//!             let grid = GridOptions::<JsonData>::new()
//!                 .column_defs(cols)
//!                 .row_model_type(RowModelType::Infinite)
//!                 .datasource(data_source)
//!                 .build(grid_div);
//!
//!             // `grid` now provides a handle to the grid and column APIs
//!             || ()
//!         },
//!         (),
//!     );
//!
//!     html! {
//!         <>
//!             <div id="grid-div" class="ag-theme-alpine" style="height: 500px"/>
//!         </>
//!     }
//! }
//!
//! #[derive(ToJsValue, Deserialize)]
//! struct JsonData {
//!     athlete: String,
//!     age: Option<usize>,
//!     country: String,
//!     year: usize,
//! }
//!
//! fn get_element_by_id(id: &str) -> HtmlElement {
//!     web_sys::window()
//!         .expect("unable to get window object")
//!         .document()
//!         .expect("unable to get document object")
//!         .get_element_by_id(id)
//!         .expect("unable to find grid-div")
//!         .dyn_into::<HtmlElement>()
//!         .unwrap()
//! }
//! ```
//!
//! [`AG Grid`]: https://www.ag-grid.com/javascript-data-grid/

mod column;
mod grid;
mod gridoptions;
mod types;

//#[doc(inline)]
pub use ag_grid_core::convert;
#[doc(hidden)]
pub use ag_grid_core::imports;
pub use ag_grid_derive::ToJsValue;
pub use column::{ColumnApi, ColumnDef};
pub use grid::{Grid, GridApi};
pub use gridoptions::GridOptions;
pub use types::{
    DataSource, DataSourceBuilder, Filter, GetRowsParams, HeaderValueGetterParams, LockPosition,
    PinnedPosition, PopupPosition, RowModelType, SortDirection, SortMethod, SortModelItem,
};
