extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::export::Span;
use syn::{parse_macro_input, Attribute, DeriveInput, IntSuffix, Lit, LitInt, Meta, MetaNameValue};

#[proc_macro_derive(Register, attributes(addr, len))]
pub fn register(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let (addr, len) = addr_len_attrs(input.attrs);

    let name = input.ident;

    let expanded = quote! {
        impl Register for #name{
            fn address(&self) -> u8 {
                #addr
            }

            fn length(&self) -> u8 {
                #len
            }
        }
    };

    TokenStream::from(expanded)
}

fn addr_len_attrs(attributes: Vec<Attribute>) -> (Lit, Lit) {
    let mut addr_lit: Option<u64> = None;
    let mut len_lit: Option<u64> = None;
    for attr in attributes.iter() {
        if let Ok(Meta::NameValue(MetaNameValue {
            ref ident, ref lit, ..
        })) = attr.parse_meta()
        {
            if ident == "addr" {
                if let Lit::Int(lit) = lit {
                    addr_lit = Some(lit.value());
                }
            }
            if ident == "len" {
                if let Lit::Int(lit) = lit {
                    len_lit = Some(lit.value())
                }
            }
        }
    }
    let addr = Lit::Int(LitInt::new(
        addr_lit.unwrap(),
        IntSuffix::U8,
        Span::call_site(),
    ));
    let len = Lit::Int(LitInt::new(
        len_lit.unwrap(),
        IntSuffix::U8,
        Span::call_site(),
    ));
    (addr, len)
}
