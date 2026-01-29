use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Fields, parse_macro_input};

use crate::{
    model::{BuilderField, BuilderStruct},
    utils::parse_builder_attrs,
};

pub fn expand(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let model = match parse_struct(input) {
        Ok(m) => m,
        Err(e) => return e.into_compile_error().into(),
    };

    generate_builder(model).into()
}

fn parse_struct(input: DeriveInput) -> syn::Result<BuilderStruct> {
    let name = input.ident;
    let generics = input.generics;
    let fields = match input.data {
        Data::Struct(ds) => match ds.fields {
            Fields::Named(named) => {
                let mut result = Vec::new();
                for f in named.named {
                    let ident = f.ident.unwrap();
                    let ty = f.ty;

                    let (skip, default) = parse_builder_attrs(&f.attrs)?;

                    result.push(BuilderField {
                        name: ident,
                        ty,
                        skip,
                        default,
                    });
                }
                result
            }
            _ => {
                return Err(syn::Error::new_spanned(
                    name,
                    "Builder only supports structs with named fields",
                ));
            }
        },
        _ => {
            return Err(syn::Error::new_spanned(
                name,
                "Builder can only be derived for structs",
            ));
        }
    };

    Ok(BuilderStruct {
        name,
        generics,
        fields,
    })
}

fn generate_builder(model: BuilderStruct) -> proc_macro2::TokenStream {
    let struct_name = model.name;
    let builder_name = format_ident!("{}Builder", struct_name);

    let generics = model.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let builder_fields = model.fields.iter().filter(|f| !f.skip).map(|f| {
        let name = &f.name;
        let ty = &f.ty;
        quote! {
            #name: ::std::option::Option<#ty>
        }
    });

    let builder_init = model.fields.iter().filter(|f| !f.skip).map(|f| {
        let name = &f.name;
        quote! {
            #name: ::std::option::Option::None
        }
    });

    let setters = model.fields.iter().filter(|f| !f.skip).map(|f| {
        let name = &f.name;
        let ty = &f.ty;
        quote! {
            pub fn #name(mut self, value: #ty) -> Self {
                self.#name = ::std::option::Option::Some(value);
                self
            }
        }
    });

    let build_fields = model.fields.iter().map(|f| {
        let name = &f.name;
        if f.skip {
            if let Some(default) = &f.default {
                quote! { #name: #default }
            } else {
                quote! { #name: ::std::default::Default::default() }
            }
        } else if let Some(default) = &f.default {
            quote! {
                #name: self.#name.unwrap_or_else(|| #default)
            }
        } else {
            quote! {
                #name: self.#name.ok_or_else(|| format!("field `{}` not set", stringify!(#name)))?
            }
        }
    });

    quote! {
        pub struct #builder_name #generics #where_clause {
            #( #builder_fields, )*
        }

        impl #impl_generics #builder_name #ty_generics #where_clause {
            #( #setters )*

            pub fn build(self) -> ::std::result::Result<#struct_name #ty_generics, ::std::string::String> {
                Ok(#struct_name {
                    #( #build_fields, )*
                })
            }
        }

        impl #impl_generics #struct_name #ty_generics #where_clause {
            pub fn builder() -> #builder_name #ty_generics {
                #builder_name {
                    #( #builder_init, )*
                }
            }
        }
    }
}
