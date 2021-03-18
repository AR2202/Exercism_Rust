extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Planet)]
pub fn planet_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_planet(&ast)
}

fn impl_planet(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Planet for #name {
            fn years_during(d: &Duration) -> f64 {
                d.s /(31557600.0 * #name::YEAR)
            }
        }
    };
    gen.into()
}
