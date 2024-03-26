use core::panic;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, Data, DeriveInput, Fields, GenericArgument, Type,
};

#[proc_macro_derive(IdentifiableImpl, attributes(identifiable_field))]
pub fn identifiable_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let fields = if let Data::Struct(syn::DataStruct {
        fields: Fields::Named(syn::FieldsNamed { ref named, .. }),
        ..
    }) = input.data
    {
        named
    } else {
        panic!("Only structs with named fields are supported")
    };

    let id_fields = fields
        .iter()
        .filter(|f| {
            f.attrs
                .iter()
                .any(|f| f.path().is_ident("identifiable_field"))
        })
        .map(|f| {
            let name = &f.ident;

            let ty = &f.ty;
            if let syn::Type::Path(ref p) = ty {
                let seg = &p.path.segments[0];

                match seg.arguments {
                    syn::PathArguments::AngleBracketed(ref inner) => {
                        let inner = &inner.args[0];
                        if let GenericArgument::Type(Type::Path(syn::TypePath { .. })) = inner {
                            // let inner_ty = &path.segments[0].ident;

                            match seg.ident.to_string().as_str() {
                                "Vec" => {
                                    quote!(
                                        for item in &self.#name {
                                            identifiers.extend(item.identifiers());
                                        }
                                    )
                                }
                                "Option" => {
                                    quote!(
                                        if let Some(inner) = &self.#name {
                                            identifiers.extend(inner.identifiers());
                                        }
                                    )
                                }
                                "Box" => {
                                    quote!(
                                        identifiers.extend(self.#name.identifiers());
                                    )
                                }
                                _ => {
                                    panic!("Only Vec, Option, and Box are supported as outer types")
                                }
                            }
                        } else {
                            panic!("Outer type must have a single inner type")
                        }
                    }
                    syn::PathArguments::None => {
                        quote!(
                            identifiers.extend(self.#name.identifiers());
                        )
                    }
                    _ => {
                        panic!("Only Vec, Option, and Box are supported")
                    }
                }
            } else {
                panic!("Only fields with a type path are supported")
            }
        });

    let expanded = quote! {
        impl Identifiable for #name {
            fn identifiers(&self) -> Vec<Identifier> {
                let mut identifiers = Vec::new();
                #( #id_fields )*
                identifiers
            }
        }
    };

    TokenStream::from(expanded)
}
