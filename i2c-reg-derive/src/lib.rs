//! Macros for register traits

#![recursion_limit = "128"]

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::export::Span;
use syn::{parse_macro_input, Attribute, DeriveInput, IntSuffix, Lit, LitInt, Meta, MetaNameValue};

/// Derive Register trait with specific address and size
///
/// # Attributes
///
/// * `address` - Register address
/// * `size` - Register number of bytes
#[proc_macro_derive(Register, attributes(address, size))]
pub fn register(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let (address, size) = address_size_attrs(input.attrs);

    let name = input.ident;

    let expanded = quote! {
        impl Register for #name{

            type Raw = [u8; #size];

            fn address(&self) -> u8 {
                #address
            }

            fn size(&self) -> usize {
                #size
            }
        }
    };

    TokenStream::from(expanded)
}

/// Derive I2cReadRegister to allow reading from register
///
/// # Attributes
///
/// * `address` - Register address
/// * `size` - Register number of bytes
#[proc_macro_derive(I2cReadRegister, attributes(address, size))]
pub fn i2c_read_register(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let (address, size) = address_size_attrs(input.attrs);

    let name = input.ident;

    let expanded = quote! {
        impl I2cReadRegister<[u8; #size]> for #name {
            fn i2c_read<I2C, Err>(&self, i2c: &mut I2C, device_address: u8) -> Result<[u8; #size], Err>
            where
                I2C: i2c::WriteRead<Error = Err>,
            {
                let mut buff = [0; #size];
                i2c.write_read(device_address, &[#address], &mut buff)?;
                Ok(buff)
            }
        }
    };

    TokenStream::from(expanded)
}

/// Derive I2cWriteRegister to allow writing to register
///
/// # Attributes
///
/// * `address` - Register address
/// * `size` - Register number of bytes
#[proc_macro_derive(I2cWriteRegister, attributes(address, size))]
pub fn i2c_write_register(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let (address, size) = address_size_attrs(input.attrs);

    let name = input.ident;

    let expanded = quote! {
        impl I2cWriteRegister<[u8; #size]> for #name {
            fn i2c_write<I2C, Err>(&self, i2c: &mut I2C, device_address: u8, raw: [u8; #size]) -> Result<(), Err>
            where
                I2C: i2c::Write<Error = Err>,
            {
                let mut payload = [0; #size + 1];
                payload[0] = #address;
                for (i, item) in raw.iter().enumerate() {
                    payload[i + 1] = *item;
                }
                i2c.write(device_address, &payload)
            }
        }
    };

    TokenStream::from(expanded)
}

fn address_size_attrs(attributes: Vec<Attribute>) -> (Lit, Lit) {
    let mut address_lit: Option<u64> = None;
    let mut size_lit: Option<u64> = None;
    for attr in attributes.iter() {
        if let Ok(Meta::NameValue(MetaNameValue {
            ref ident, ref lit, ..
        })) = attr.parse_meta()
        {
            if ident == "address" {
                if let Lit::Int(lit) = lit {
                    address_lit = Some(lit.value());
                }
            }
            if ident == "size" {
                if let Lit::Int(lit) = lit {
                    size_lit = Some(lit.value())
                }
            }
        }
    }
    let address = Lit::Int(LitInt::new(
        address_lit.unwrap(),
        IntSuffix::U8,
        Span::call_site(),
    ));
    let size = Lit::Int(LitInt::new(
        size_lit.unwrap(),
        IntSuffix::Usize,
        Span::call_site(),
    ));
    (address, size)
}
