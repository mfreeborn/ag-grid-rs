use darling::{ast, FromDeriveInput, FromField};
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{DeriveInput, GenericArgument};

const UNSUPPORTED_ERROR: &str =
    r#"FromInterface can only be derived for structs with named fields"#;

pub(crate) fn from_interface_impl(input: DeriveInput) -> proc_macro::TokenStream {
    let struct_receiver = match StructReceiver::from_derive_input(&input) {
        Ok(r) => r,
        Err(e) => {
            return proc_macro::TokenStream::from(
                darling::Error::custom(format!("{}. {}", UNSUPPORTED_ERROR, e)).write_errors(),
            )
        }
    };
    quote! {
        #struct_receiver
    }
    .into()
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(field_setter), supports(struct_named))]
struct StructReceiver {
    /// The struct name.
    ident: syn::Ident,

    /// The body of the struct or enum. We don't care about enum fields
    /// because we accept only named structs. Hence the first type is null.
    data: ast::Data<(), FieldReceiver>,
}

impl ToTokens for StructReceiver {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = &self.ident;
        let interface_ident = Ident::new(&format!("I{}", ident), proc_macro2::Span::call_site());

        let mut assignments = quote![];

        match self.data {
            ast::Data::Struct(ref f) => {
                for field in f.fields.iter() {
                    let field_name = field.ident.as_ref().unwrap();

                    let (type_ident, ident_args) = match field.ty {
                        syn::Type::Path(ref type_path) => {
                            let path_end = type_path
                                .path
                                .segments
                                .last()
                                .expect("type path segments is empty");

                            (path_end.ident.to_string(), path_end.arguments.clone())
                        }
                        _ => unimplemented!("only `syn::Type::Path`s are supported for struct field types at present"),
                    };

                    let interface_getter = if type_ident == *"Vec" {
                        // we need to extract the generic type from the Vec
                        let inner_type = match ident_args {
                            syn::PathArguments::AngleBracketed(args) => {
                                let arg = args.args.first().expect("expected a syn::Type::Path to be the first element in the args list");
                                match arg {
                                    GenericArgument::Type(syn::Type::Path(type_path)) => {
                                        let path_end = type_path
                                            .path
                                            .segments
                                            .last()
                                            .expect("type path segments is empty");

                                        let mut inner_type_ident = path_end.ident.clone();
                                        inner_type_ident.set_span(proc_macro2::Span::call_site());

                                        quote!(#inner_type_ident)
                                    }
                                    _ => unimplemented!(),
                                }
                            }
                            _ => unimplemented!(),
                        };

                        quote!(#field_name().iter().map(#inner_type::from).collect())
                    } else {
                        quote!(#field_name())
                    };

                    let assignment = quote!(
                        #field_name: i.#interface_getter,
                    );
                    assignments.append_all(assignment);
                }
            }
            // We only support structs
            _ => unreachable!(),
        }

        tokens.append_all(quote! {
            impl From<&#interface_ident> for #ident {
                fn from(i: &#interface_ident) -> Self {
                    Self {
                        #assignments
                    }
                }
            }
        });
    }
}

#[derive(Debug, FromField)]
#[darling(attributes(field_setter), forward_attrs(doc, serde))]
struct FieldReceiver {
    /// Name of the field
    ident: Option<syn::Ident>,

    /// The type of the field
    ty: syn::Type,
}
