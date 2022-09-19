use convert_case::{Case, Casing};
use darling::{ast, FromDeriveInput, FromField, FromMeta, FromVariant};
use proc_macro2::TokenStream;
use proc_macro_crate::{crate_name, FoundCrate};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{DeriveInput, Generics, Type};

fn root_crate() -> TokenStream {
    let found_crate = crate_name("ag-grid-rs").expect("ag-grid-rs is present in `Cargo.toml`");
    match found_crate {
        FoundCrate::Itself => quote!(ag_grid_core),
        FoundCrate::Name(_) => quote!(ag_grid_rs),
    }
}

const UNSUPPORTED_ERROR: &str =
    r#"ToJsValue can only be derived for structs with named fields or enums"#;

pub(crate) fn to_js_value_impl(input: DeriveInput) -> proc_macro::TokenStream {
    let receiver = match Receiver::from_derive_input(&input) {
        Ok(r) => r,
        Err(e) => {
            return proc_macro::TokenStream::from(
                darling::Error::custom(format!("{}. {}", UNSUPPORTED_ERROR, e)).write_errors(),
            )
        }
    };
    quote! {
        #receiver
    }
    .into()
}

#[derive(Debug, FromDeriveInput)]
struct Receiver {
    ident: syn::Ident,
    data: ast::Data<VariantReceiver, FieldReceiver>,
    generics: Generics,

    /// When applied to structs, fields of type `Option<T>` are not serialized.
    /// By default, `Option<T>` will serialize to `JsValue::null()` if the value
    /// is `None`.
    #[darling(default)]
    skip_serializing_none: bool,
}

impl ToTokens for Receiver {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let root_crate = root_crate();
        let ident = &self.ident;
        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();

        let body = match self.data {
            ast::Data::Struct(ref f) => {
                let mut serialized_fields = quote![];

                for field in f.fields.iter() {
                    serialized_fields.append_all(field.serialize(self.skip_serializing_none));
                }

                quote! {
                    use #root_crate::convert::ToJsValue;
                    let obj = #root_crate::imports::Object::new();
                    #serialized_fields
                    obj.into()
                }
            }
            ast::Data::Enum(ref v) => {
                let mut arms = quote![];

                for v in v {
                    arms.append_all(v.serialize());
                }

                quote! {
                    match *self {
                        #arms
                    }
                }
            }
        };

        tokens.append_all(quote! {
            impl #impl_generics #root_crate::convert::ToJsValue for #ident #ty_generics #where_clause {
                fn to_js_value(&self) -> wasm_bindgen::JsValue {
                    #body
                }
            }
        });
    }
}

#[derive(Debug, FromField)]
#[darling(attributes(js_value))]
struct FieldReceiver {
    ident: Option<syn::Ident>,
    ty: syn::Type,

    /// Allow individual fields to have their serialized name over-ridden
    rename: Option<String>,
}

impl FieldReceiver {
    fn serialize(&self, skip_serializing_none: bool) -> TokenStream {
        let field_ident = self.ident.as_ref().unwrap();
        let js_name = match self.rename.as_ref() {
            Some(name) => name.clone(),
            None => field_ident.to_string().to_case(Case::Camel),
        };

        let is_option = is_option(&self.ty);

        let mut out = quote![];
        if is_option {
            out.append_all(quote! {
                if let Some(#field_ident) = &self.#field_ident {
                    obj.set(&#js_name, #field_ident.to_js_value());
                }
            });

            if !skip_serializing_none {
                out.append_all(quote! {
                    else {
                        obj.set(&#js_name, wasm_bindgen::JsValue::null());
                    }
                });
            }
        } else {
            out.append_all(quote! {obj.set(&#js_name,
            self.#field_ident.to_js_value());})
        }

        out
    }
}

#[derive(FromMeta, Debug)]
enum AltValue {
    Null,
    True,
    False,
    Undefined,
}

#[derive(FromVariant, Debug)]
#[darling(attributes(js_value))]
struct VariantReceiver {
    ident: syn::Ident,

    /// Allow individual variants to have their serialized name over-ridden
    rename: Option<String>,

    /// Allow an override for certain primitive JsValues
    serialize_as: Option<AltValue>,
}

impl VariantReceiver {
    fn serialize(&self) -> TokenStream {
        let variant_ident = &self.ident;
        let js_name = match self.rename.as_ref() {
            Some(name) => name.clone(),
            None => variant_ident.to_string().to_case(Case::Camel),
        };

        let serialized_value = self
            .serialize_as
            .as_ref()
            .map(|var| match var {
                AltValue::Null => quote! {wasm_bindgen::JsValue::null()},
                AltValue::Undefined => quote! {wasm_bindgen::JsValue::undefined()},
                AltValue::True => quote! {wasm_bindgen::JsValue::from_bool(true)},
                AltValue::False => quote! {wasm_bindgen::JsValue::from_bool(false)},
            })
            .unwrap_or_else(|| quote! {wasm_bindgen::JsValue::from_str(#js_name)});

        quote! {
            Self::#variant_ident => #serialized_value,
        }
    }
}

/// Return `true`, if the type path refers to `std::option::Option`
///
/// Accepts
///
/// * `Option`
/// * `std::option::Option`, with or without leading `::`
/// * `core::option::Option`, with or without leading `::`
///
/// Implementation copied from https://github.com/jonasbb/serde_with
fn is_option(type_: &Type) -> bool {
    match type_ {
        Type::Array(_)
        | Type::BareFn(_)
        | Type::ImplTrait(_)
        | Type::Infer(_)
        | Type::Macro(_)
        | Type::Never(_)
        | Type::Ptr(_)
        | Type::Reference(_)
        | Type::Slice(_)
        | Type::TraitObject(_)
        | Type::Tuple(_)
        | Type::Verbatim(_) => false,

        Type::Group(syn::TypeGroup { elem, .. })
        | Type::Paren(syn::TypeParen { elem, .. })
        | Type::Path(syn::TypePath {
            qself: Some(syn::QSelf { ty: elem, .. }),
            ..
        }) => is_option(elem),

        Type::Path(syn::TypePath { qself: None, path }) => {
            (path.leading_colon.is_none()
                && path.segments.len() == 1
                && path.segments[0].ident == "Option")
                || (path.segments.len() == 3
                    && (path.segments[0].ident == "std" || path.segments[0].ident == "core")
                    && path.segments[1].ident == "option"
                    && path.segments[2].ident == "Option")
        }
        _ => false,
    }
}
