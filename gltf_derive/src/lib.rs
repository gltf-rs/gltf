
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Adapted from `validator_derive` (https://github.com/Keats/validator).
//
// See LICENSE for details.

#![recursion_limit = "128"]

extern crate inflections;
#[macro_use]
extern crate quote;
extern crate proc_macro;
extern crate syn;

use proc_macro::TokenStream;

#[proc_macro_derive(Validate)]
pub fn main(input: TokenStream) -> TokenStream {
    let source = input.to_string();
    let ast = syn::parse_macro_input(&source).unwrap();
    let tokens = expand(&ast);
    tokens.parse().unwrap()
}

fn expand(ast: &syn::MacroInput) -> quote::Tokens {
    let fields = match ast.body {
        syn::Body::Struct(syn::VariantData::Struct(ref fields)) => fields,
        _ => panic!("#[derive(Validate)] only works on `struct`s"),
    };
    let ident = &ast.ident;
    let validations: Vec<quote::Tokens> = fields.iter()
        .map(|f| f.ident.as_ref().unwrap())
        .map(|ident| {
                 use inflections::Inflect;
                 let field = ident.as_ref().to_camel_case();
                 quote!(self.#ident.validate(root, || path().field(#field), report))
             })
        .collect();
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    quote!(
        impl #impl_generics ::validation::Validate<'a> for #ident #ty_generics #where_clause {
            fn validate<P, R>(&self, root: &::json::Root<'a>, path: P, report: &mut R)
                where P: Fn() -> ::validation::JsonPath, R: FnMut(::validation::Error),
            {
                #(
                    #validations;
                )*
            }
        }
    )
}
