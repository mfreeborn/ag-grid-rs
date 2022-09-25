use ag_grid_derive::ToJsValue;

/// Allowed values for [`ColumnDef::sort`][crate::ColumnDef::sort] and related
/// methods.
#[derive(ToJsValue)]
pub enum SortMethod {
    Asc,
    Desc,
    #[js_value(serialize_as = "null")]
    Null,
}
