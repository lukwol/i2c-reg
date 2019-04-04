#![recursion_limit = "128"]

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

            fn length(&self) -> usize {
                #len
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(I2cReadRegister, attributes(addr, len))]
pub fn i2c_read_register(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let (_, len) = addr_len_attrs(input.attrs);

    let name = input.ident;

    let expanded = quote! {
        impl<'a> I2cReadRegister<'a, [u8; #len]> for #name {
            fn i2c_read<I2C, Err>(&self) -> &Fn(&mut I2C, u8, u8) -> Result<[u8; #len], Err>
            where
                I2C: i2c::WriteRead<Error = Err>,
            {
                &|i2c, device_address, reg_address| {
                    let mut buff = [0; #len];
                    i2c.write_read(device_address, &[reg_address], &mut buff)?;
                    Ok(buff)
                }
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(I2cWriteRegister, attributes(addr, len))]
pub fn i2c_write_register(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let (_, len) = addr_len_attrs(input.attrs);

    let name = input.ident;

    let expanded = quote! {
        impl<'a> I2cWriteRegister<'a, [u8; #len]> for #name {
            fn i2c_write<I2C, Err>(&self) -> &Fn(&mut I2C, u8, u8, [u8; #len]) -> Result<(), Err>
            where
                I2C: i2c::Write<Error = Err>,
            {
                &|i2c, device_address, reg_address, value| {
                    let mut payload = [0; #len + 1];
                    payload[0] = reg_address;
                    for (i, item) in value.iter().enumerate() {
                        payload[i + 1] = *item;
                    }
                    i2c.write(device_address, &payload)
                }
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
        IntSuffix::Usize,
        Span::call_site(),
    ));
    (addr, len)
}
