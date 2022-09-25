use convert_case::{Case, Casing};
use darling::{ast, FromDeriveInput, FromField};
use proc_macro2::{Ident, TokenStream};
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

#[allow(clippy::large_enum_variant, clippy::enum_variant_names)]
#[derive(Clone)]
enum FieldType {
    OptionString,
    OptionOneOrManyString,
    OptionClosure,
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
            ["OneOrMany", "String"] => FieldType::OptionOneOrManyString,
            ["Closure", _] => FieldType::OptionClosure,
            _ => FieldType::OptionOther(types.get(1).cloned().unwrap()),
        }
    }
}

#[derive(Debug, FromField)]
#[darling(attributes(field_setter), forward_attrs(doc))]
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
        // Safe to unwrap because this macro can only be used on structs with named
        // fields
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

        let (mutability, value_type, value_convert, array_value_convert) = match &field_type {
            FieldType::OptionClosure => {
                // We have two choices. We can either spend the rest of eternity parsing the
                // exact names of the inputs and outputs of the closure... or we can just assume
                // the following convention.
                let pascal_cased = field_ident.to_string().to_case(Case::Pascal);
                let js_params = format!("I{pascal_cased}Params");
                let js_params = Ident::new(&js_params, proc_macro2::Span::call_site());
                let params = format!("{pascal_cased}Params");
                let params = Ident::new(&params, proc_macro2::Span::call_site());
                (
                    quote![mut],
                    quote![impl FnMut(crate::callbacks::#params) -> String + 'static],
                    quote![wasm_bindgen::closure::Closure::<
                        dyn FnMut(crate::callbacks::#js_params) -> String,
                    >::new(
                        move |js_params: crate::callbacks::#js_params| value(
                            (&js_params).into()
                        )
                    )],
                    quote![],
                )
            }
            FieldType::OptionString => (
                quote![],
                quote![impl AsRef<str>],
                quote![value.as_ref().to_owned()],
                quote![],
            ),
            FieldType::OptionOneOrManyString => (
                quote![],
                quote![impl AsRef<str>],
                quote![value.as_ref().to_string().into()],
                quote![value
                    .iter()
                    .map(|v| v.as_ref().to_string())
                    .collect::<Vec<_>>()
                    .into()],
            ),
            FieldType::OptionOther(inner_ty) => {
                (quote![], quote![#inner_ty], quote![value], quote![])
            }
        };

        let setter = quote! {
            #field_docs
            pub fn #field_ident(mut self, #mutability value: #value_type) -> Self {
                self.#field_ident = Some(#value_convert);
                self
            }
        };

        let array_setter = match field_type {
            FieldType::OptionOneOrManyString => {
                let array_ident = Ident::new(
                    &format!("{}_array", field_ident.to_string().trim_end_matches('_')),
                    proc_macro2::Span::call_site(),
                );
                quote! {
                    #field_docs
                    pub fn #array_ident(mut self, value: Vec<#value_type>) -> Self {
                        self.#field_ident = Some(#array_value_convert);
                        self
                    }
                }
            }
            _ => quote![],
        };

        quote![
            #setter
            #array_setter
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
