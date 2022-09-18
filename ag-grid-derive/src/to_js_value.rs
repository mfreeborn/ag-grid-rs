use convert_case::{Case, Casing};
use darling::{
    ast::{self, Fields},
    FromDeriveInput, FromField, FromVariant,
};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{DeriveInput, Generics};

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
}

impl ToTokens for Receiver {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = &self.ident;
        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();

        let body = match self.data {
            ast::Data::Struct(ref f) => {
                let mut if_let_blocks = quote![];

                for field in f.fields.iter() {
                    if_let_blocks.append_all(field.if_let_block());
                }

                quote! {
                    use crate::traits::ToJsValue;
                    use wasm_bindgen::JsCast;
                    let obj = crate::utils::Object::new();
                    #if_let_blocks
                    obj.into()
                }
            }
            ast::Data::Enum(ref v) => {
                let mut arms = quote![];

                for v in v {
                    let variant_ident = &v.ident;
                    let camel_cased = &v.ident.to_string().to_case(Case::Camel);
                    let fields = &v.fields;

                    // TODO: handle enums with non-unit variants
                    if fields.is_empty() {
                        arms.append_all(quote! {
                            Self::#variant_ident => wasm_bindgen::JsValue::from_str(#camel_cased),
                        });
                    }
                }

                quote! {
                    match *self {
                        #arms
                    }
                }
            }
        };

        tokens.append_all(quote! {
            impl #impl_generics crate::traits::ToJsValue for #ident #ty_generics #where_clause {
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

    rename: Option<String>,
}

impl FieldReceiver {
    fn if_let_block(&self) -> TokenStream {
        let field_ident = self.ident.as_ref().unwrap();
        let js_name = match &self.rename {
            Some(name) => name.clone(),
            None => field_ident.to_string().to_case(Case::Camel),
        };

        quote! {
            if let Some(#field_ident) = &self.#field_ident {
                obj.set(&#js_name, #field_ident.to_js_value())
            }
        }
    }
}

#[derive(FromField, Debug)]
struct VariantFieldReceiver {
    _ident: Option<syn::Ident>,
}

#[derive(FromVariant, Debug)]
struct VariantReceiver {
    ident: syn::Ident,
    fields: Fields<VariantFieldReceiver>,
}
