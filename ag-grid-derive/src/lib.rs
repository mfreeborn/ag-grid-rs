use syn::DeriveInput;

mod field_setter;
mod from_interface;
mod to_js_value;

#[proc_macro_derive(FieldSetter, attributes(field_setter))]
pub fn field_setter(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse::<DeriveInput>(input).unwrap();
    field_setter::field_setter_impl(input)
}

/// Automatically derive the `ToJsValue` trait to enable the annotated type to
/// be serialized to a `wasm_bindgen::JsValue`.
///
/// The macro can be applied either to structs with named fields, or fieldless
/// enums.
///
/// # Examples
///
/// ## Structs with named fields
///
/// The macro can be derived for any struct with named fields where all field
/// types also implement `ToJsValue`. Most primitive types have a `ToJsValue`
/// implemntation already. Given the following struct,
///
/// ```rust
/// #[derive(ToJsValue)]
/// struct Data {
///     first_value: String,
///     second_value: bool
/// }
/// ```
///
/// the following equivalent implementation would be generated:
///
/// ```rust
/// impl ToJsValue for Data {
///     fn to_js_value(&self) -> JsValue {
///         let obj = js_sys::Object::new();
///         // We actually use a custom implementation of Object which has a helper `set` method.
///         obj.set("firstValue", self.first_value.to_js_value());
///         obj.set("secondValue", self.second_value.to_js_value());
///         obj.into()
///     }
/// }
/// ```
///
/// Note that the field names are converted to camelCase by default.
///
/// ### Supported attributes
///
/// At the the container-level:
/// * `#[js_value(skip_serializing_none)]` - any top-level `Option<T>` which has
///   a value of `None` will be omitted from the serialized object. When this
///   flag is absent, i.e. by default, `None` values are serialized to
///   `JsValue::null()`.
///
/// At the field-level:
/// * `#[js_value(rename = "...")]` - override the default camelCase name for
///   the serialized field.
///
/// ## Fieldless enums
///
/// A fieldless enum, such as
///
/// ```rust
/// derive(ToJsValue)
/// enum MoonPhase {
///     New,
///     FirstQuarter,
///     ThirdQuarter,
///     Full     
/// }
/// ```
///
/// would produce an implementation equivalent to:
///
/// ```rust
/// impl ToJsValue for MoonPhase {
///     fn to_js_value(&self) -> JsValue{
///         Self::New => JsValue::from_str("new"),
///         Self::New => JsValue::from_str("firstQuarter"),
///         Self::New => JsValue::from_str("ThirdQuarter"),
///         Self::New => JsValue::from_str("full"),
///     }
/// }
/// ```
///
/// As with structs the default behaviour is to serialize the variant to
/// camelCase.
///
/// ### Supported attributes
///
/// At the variant-level:
/// * `#[js_value(rename = "...")]` - override the default camelCase name for
///   the serialized variant.
/// * `#[js_value(serialize_as = "{placeholder}")` - , where `{placeholder}` can
///   be one of : `null`, `undefined`, `true` or `false`. Instead of the tagged
///   variant being serialized to a string, it is instead serialized to one of
///   the chosen literal JavaScript Values. It is useful if, for example, you
///   have an enum with a `Null` variant and want it to be serialized to a a
///   primitive `null` rather than the string `"null"`.
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
