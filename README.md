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

Standalone examples can be found in the [ag-grid-rs/examples/](https://github.com/mfreeborn/ag-grid-rs/tree/main/examples) directory, and a basic example using the `Yew` frontend framework is shown below.

First, make sure you have the JavaScript AG Grid library available to the web page by including a CDN link in your base HTML page:

```html
// index.html

<!doctype html>
<html lang="en">
    <head>
        <!-- snip -->
        <script src="https://unpkg.com/ag-grid-community/dist/ag-grid-community.min.js"></script>
    </head>
    <!-- snip -->
</html>
```

Then, in your application:

```rust
use ag_grid_rs::{
    gridoptions::{DataSourceBuilder, RowModelType},
    ColumnDef, GridOptions, ToJsValue, 
};
use gloo_net::http::Request;
use serde::Deserialize;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::prelude::*;

#[function_component(Table)]
pub fn table() -> Html {
    // Fire the hook just once on initial load
    use_effect_with_deps(
        |_| {
            // Get the element to which you want to attach the grid
            let grid_div = get_element_by_id("grid-div");
            // Define your columns
            let field_names = vec!["athlete", "age", "country", "year"];
            let cols = field_names
                .iter()
                .map(|name| ColumnDef::new().field(name).sortable(true))
                .collect();

            // Create your datasource, including a closure that will return rows from the
            // server
            let data_source = DataSourceBuilder::new(|params| async move {
                // `params` contains information from AG Grid about which rows to get, how to
                // sort the data, etc
                let data_url = "https://www.ag-grid.com/example-assets/olympic-winners.json";
                let rows = Request::get(data_url)
                    .send()
                    .await?
                    .json::<Vec<JsonData>>()
                    .await?;

                Ok((rows, None))
            })
            .build();

            let grid = GridOptions::<JsonData>::new()
                .column_defs(cols)
                .row_model_type(RowModelType::Infinite)
                .datasource(data_source)
                .build(grid_div);

            // `grid` now provides a handle to the grid and column APIs
            || ()
        },
        (),
    );

    html! {
        <>
            <div id="grid-div" class="ag-theme-alpine" style="height: 500px"/>
        </>
    }
}

#[derive(ToJsValue, Deserialize)]
struct JsonData {
    athlete: String,
    age: Option<usize>,
    country: String,
    year: usize,
}

fn get_element_by_id(id: &str) -> HtmlElement {
    web_sys::window()
        .expect("unable to get window object")
        .document()
        .expect("unable to get document object")
        .get_element_by_id(id)
        .expect("unable to find grid-div")
        .dyn_into::<HtmlElement>()
        .unwrap()
}
```