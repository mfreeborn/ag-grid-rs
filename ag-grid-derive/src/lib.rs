use syn::DeriveInput;

mod field_setter;
mod from_interface;
mod to_js_value;

#[proc_macro_derive(FieldSetter, attributes(field_setter))]
pub fn field_setter(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse::<DeriveInput>(input).unwrap();
    field_setter::field_setter_impl(input)
}

#[proc_macro_derive(ToJsValue, attributes(js_value))]
pub fn to_js_value(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse::<DeriveInput>(input).unwrap();
    to_js_value::to_js_value_impl(input)
}

#[proc_macro_derive(FromInterface)]
pub fn from_interface(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse::<DeriveInput>(input).unwrap();
    from_interface::from_interface_impl(input)
}
