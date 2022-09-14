use darling::{ast, FromDeriveInput, FromField};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{DeriveInput, GenericArgument, Generics, TypeParamBound};

const UNSUPPORTED_ERROR: &str = r#"FieldSetter can only be derived for structs with named fields"#;

pub(crate) fn field_setter_impl(input: DeriveInput) -> proc_macro::TokenStream {
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

    generics: Generics,
}

impl ToTokens for StructReceiver {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = &self.ident;
        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();

        let mut setter_functions = quote![];
        let mut default_vals = quote![];

        match self.data {
            ast::Data::Struct(ref f) => {
                for field in f.fields.iter() {
                    let setter = field.setter();
                    setter_functions.append_all(setter);
                    default_vals.append_all(field.default_value());
                }
            }
            // We only support structs
            _ => unreachable!(),
        }

        tokens.append_all(quote! {
            impl #impl_generics Default for #ident #ty_generics #where_clause {
                fn default() -> Self {
                    Self {
                        #default_vals
                    }
                }
            }

            impl #impl_generics #ident #ty_generics #where_clause {
                #setter_functions
            }
        });
    }
}

#[derive(Clone)]
enum FieldType {
    OptionString,
    OptionOther(syn::Type),
}

fn _type_str_parts(field_type: &syn::Type) -> (Vec<String>, Vec<syn::Type>) {
    let mut type_ = field_type;

    let mut parts = Vec::new();
    let mut types = vec![type_.clone()];

    loop {
        match type_ {
            syn::Type::Path(ref type_path) if type_path.qself == None => {
                if let Some(segment) = type_path.path.segments.last() {
                    parts.push(segment.ident.to_string());
                    match &segment.arguments {
                        syn::PathArguments::AngleBracketed(args) => match args.args.first() {
                            Some(first) => {
                                if let GenericArgument::Type(inner_ty) = first {
                                    type_ = inner_ty;
                                    types.push(type_.clone());
                                } else {
                                    break;
                                }
                            }
                            None => break,
                        },
                        _ => break,
                    }
                } else {
                    break;
                }
            }

            syn::Type::TraitObject(ref obj) => {
                if obj.dyn_token.is_some() {
                    if let Some(TypeParamBound::Trait(t)) = obj.bounds.first() {
                        if let Some(segment) = t.path.segments.last() {
                            parts.push(segment.ident.to_string());
                        }
                    }
                }
                break;
            }
            _ => break,
        }
    }
    (parts, types)
}

impl FieldType {
    fn infer(field_type: &syn::Type) -> Self {
        let (type_str_parts, types) = _type_str_parts(field_type);

        let remaining: Vec<_> = type_str_parts.iter().skip(1).map(|x| x.as_str()).collect();

        match remaining.as_slice() {
            ["String"] => FieldType::OptionString,
            _ => FieldType::OptionOther(types.get(1).cloned().unwrap()),
        }
    }
}

#[derive(Debug, FromField)]
#[darling(attributes(field_setter), forward_attrs(doc, serde))]
struct FieldReceiver {
    /// Name of the field
    ident: Option<syn::Ident>,

    /// The type of the field
    ty: syn::Type,

    attrs: Vec<syn::Attribute>,

    #[darling(default)]
    skip: bool,
}

impl FieldReceiver {
    fn default_value(&self) -> TokenStream {
        // Safe to unwrap because this macro can only be used on structs with named fields
        let field_ident = self.ident.as_ref().unwrap();
        quote![
            #field_ident: None,
        ]
    }

    /// Generate code for the setter method
    fn setter(&self) -> TokenStream {
        if self.skip {
            return quote![];
        }

        let field_ident = self.ident.as_ref().unwrap();
        let field_type = &self.ty;
        let field_docs = self.docs();

        let field_type = FieldType::infer(field_type);

        let (value_type, value_convert) = match &field_type {
            FieldType::OptionString => (quote![impl AsRef<str>], quote![value.as_ref().to_owned()]),
            FieldType::OptionOther(inner_ty) => (quote![#inner_ty], quote![value]),
        };

        let setter = quote! {
            #field_docs
            pub fn #field_ident(mut self, value: #value_type) -> Self {
                self.#field_ident = Some(#value_convert);
                self
            }
        };

        quote![
            #setter
        ]
    }

    fn docs(&self) -> TokenStream {
        self.search_attrs("doc")
    }

    fn search_attrs(&self, name: &str) -> TokenStream {
        self.attrs
            .iter()
            .filter(|attr| {
                attr.path
                    .segments
                    .first()
                    .map_or(false, |p| p.ident == name)
            })
            .map(|attr| {
                quote![
                    #attr
                ]
            })
            .collect()
    }
}
