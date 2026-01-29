use proc_macro::TokenStream;

mod builder;
mod model;
mod utils;

#[proc_macro_derive(Builder, attributes(builder))]
pub fn derive_builder(input: TokenStream) -> TokenStream {
    builder::expand(input)
}
