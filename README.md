# ag-grid-rs &emsp; [![Build Status]][actions] [![Latest Version]][crates.io] [![Downloads]][crates.io] [![Docs]][docs.rs]

[Build Status]: https://img.shields.io/github/workflow/status/mfreeborn/ag-grid-rs/CI/main
[actions]: https://github.com/mfreeborn/ag-grid-rs/actions?query=branch%3Amain
[Latest Version]: https://img.shields.io/crates/v/ag-grid-rs.svg
[Downloads]: https://img.shields.io/crates/d/ag-grid-rs.svg
[crates.io]: https://crates.io/crates/ag-grid-rs
[Docs]: https://img.shields.io/badge/docs-latest-blue.svg
[docs.rs]: https://docs.rs/ag-grid-rs/latest/ag_grid_rs

Rust bindings for the [AG Grid](https://www.ag-grid.com/) JavaScript table library.

## Usage

ag-grid-rs aims to follow the API of AG Grid in an unsurprising way, and generally makes use of the builder pattern for constructing the Rust structures.

An example using the `Yew` frontend framework is shown below.

```rust
use ag_grid_rs::{ColumnDef, DataSourceBuilder, Filter, GridOptions, RowData, RowModelType};
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::prelude::*;

#[function_component(Grid)]
fn grid() -> Html {
    // We are using yew_hooks::use_effect_once to initialise the grid once on page
    // load
    yew_hooks::use_effect_once(|| {
        // Get the element to which you want to attach the grid
        let grid_div = get_element_by_id("grid-div");

        // Define your columns
        let col_1 = ColumnDef::new().field("make").sortable(true);
        let col_2 = ColumnDef::new()
            .field("model")
            .filter(Filter::AgTextColumnFilter)
            .floating_filter(true);

        // The Grid itself is constructed from the GridOptions builder struct
        let grid = GridOptions::new()
            .column_defs(vec![col_1, col_2])
            .row_model_type(RowModelType::Infinite)
            .pagination(true)
            .cache_block_size(100)
            // The `build` finaliser consumes the `GridOptions` and returns a `Grid` instance
            .build(grid_div);

        // Here we are showing that you can also configure the grid after it is built,
        // in the same way that you can in the JavaScript library
        let data_source = DataSourceBuilder::new(|params| async move {
            // In reality you would communicate with your backend server here to retrieve
            // the requested rows. `params` is a struct containing information
            // about which rows the frontend is requesting.
            let row = RowData::new(vec![("make", &"Jaguar"), ("model", &"F-Type")]);
            Ok(vec![row])
        })
        .build();

        // The `Grid` instance we built a few lines up provides access to the underlying
        // grid and column apis
        grid.api.set_data_source(data_source);
        || ()
    });

    html! {
        <div id="grid-div" class="ag-theme-alpine" style="height: 500px"/>
    }
}

fn get_element_by_id(id: &str) -> HtmlElement {
    web_sys::window()
        .expect("unable to get window object")
        .document()
        .expect("unable to get document object")
        .get_element_by_id("grid-div")
        .expect("unable to find grid-div")
        .dyn_into::<HtmlElement>()
        .unwrap()
}
```