use ag_grid_rs::{
    gridoptions::{DataSourceBuilder, RowModelType},
    sort::SortDirection,
    ColumnDef, GridOptions, ToJsValue,
};
use gloo_net::http::Request;
use serde::Deserialize;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
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
                log::info!("{:?}", params);
                let data_url = "https://www.ag-grid.com/example-assets/olympic-winners.json";
                let mut rows = Request::get(data_url)
                    .send()
                    .await?
                    .json::<Vec<JsonData>>()
                    .await?;

                // Typically, one would send this information to the backend to perform the
                // sorting/filtering/range selection, rather than doing it all manually here
                if let Some(sort_model) = params.sort_model.get(0) {
                    match (sort_model.col_id.as_str(), sort_model.sort) {
                        ("athlete", SortDirection::Asc) => {
                            rows.sort_by(|a, b| b.athlete.cmp(&a.athlete))
                        }
                        ("athlete", SortDirection::Desc) => {
                            rows.sort_by(|a, b| a.athlete.cmp(&b.athlete))
                        }
                        ("age", SortDirection::Asc) => rows.sort_by(|a, b| b.age.cmp(&a.age)),
                        ("age", SortDirection::Desc) => rows.sort_by(|a, b| a.age.cmp(&b.age)),
                        ("country", SortDirection::Asc) => {
                            rows.sort_by(|a, b| b.country.cmp(&a.country))
                        }
                        ("country", SortDirection::Desc) => {
                            rows.sort_by(|a, b| a.country.cmp(&b.country))
                        }
                        ("year", SortDirection::Asc) => rows.sort_by(|a, b| b.year.cmp(&a.year)),
                        ("year", SortDirection::Desc) => rows.sort_by(|a, b| a.year.cmp(&b.year)),
                        _ => unreachable!(),
                    }
                }

                let rows = rows
                    .into_iter()
                    .skip(params.start_row as usize)
                    .take(params.end_row as usize)
                    .collect();

                Ok((rows, None))
            })
            .build();

            let _grid = GridOptions::<JsonData>::new()
                .column_defs(cols)
                .row_model_type(RowModelType::Infinite)
                .datasource(data_source)
                .build(grid_div);

            // `_grid` now provides a handle to the grid and column APIs
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

#[derive(ToJsValue, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
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

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
