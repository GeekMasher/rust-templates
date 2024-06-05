//! Derive macros for the `lib` crate.

#![allow(unused_imports)]
#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;
extern crate syn;

// #[proc_macro_derive(...)]
// pub fn table_derive(input: TokenStream) -> TokenStream {
//     let ast: DeriveInput = parse_macro_input!(input as DeriveInput);

//     // ...
// }
