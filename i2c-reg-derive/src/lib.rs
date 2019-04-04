extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, IntSuffix, Lit, Meta, MetaNameValue};

#[proc_macro_derive(Register, attributes(addr))]
pub fn my_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let addr_attr = &input.attrs[0].parse_meta().unwrap();

    let mut addr_lit: Option<u64> = None;

    match addr_attr {
        Meta::NameValue(MetaNameValue {
            ref ident, ref lit, ..
        }) if ident == "addr" => {
            if let Lit::Int(lit) = lit {
                addr_lit = Some(lit.value());
            }
        }
        _ => (),
    };

    let addr_int_lit = syn::Lit::Int(syn::LitInt::new(
        addr_lit.unwrap(),
        IntSuffix::U8,
        syn::export::Span::call_site(),
    ));

    let expanded = quote! {
        impl Register for #name{
            fn address(&self) -> Address {
                Address(#addr_int_lit)
            }
        }
    };

    TokenStream::from(expanded)
}
