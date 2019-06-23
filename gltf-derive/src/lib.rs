// Adapted from `validator_derive` (https://github.com/Keats/validator).
//
// See LICENSE for details.

#![recursion_limit = "128"]

extern crate proc_macro;

use proc_macro::TokenStream;
use syn::DeriveInput;

#[proc_macro_derive(Validate)]
pub fn derive_validate(input: TokenStream) -> TokenStream {
    expand(&syn::parse_macro_input!(input as DeriveInput)).into()
}

fn expand(ast: &DeriveInput) -> proc_macro2::TokenStream {
    use proc_macro2::TokenStream;
    use quote::quote;

    let fields = match ast.data {
        syn::Data::Struct(ref data_struct) => &data_struct.fields,
        _ => panic!("#[derive(Validate)] only works on `struct`s"),
    };
    let ident = &ast.ident;
    let validations: Vec<TokenStream> = fields
        .iter()
        .map(|f| f.ident.as_ref().unwrap())
        .map(|ident| {
            use inflections::Inflect;
            let field = ident.to_string().to_camel_case();
            quote!(
                self.#ident.validate(
                    _root,
                    || _path().field(#field),
                    _report,
                )
            )
        })
        .collect();
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    quote!(
        impl #impl_generics crate::validation::Validate
            for #ident #ty_generics #where_clause
        {
            fn validate<P, R>(
                &self,
                _root: &crate::Root,
                _path: P,
                _report: &mut R
            ) where
                P: Fn() -> crate::Path,
                R: FnMut(&Fn() -> crate::Path, crate::validation::Error),
            {
                #(
                    #validations;
                )*
            }
        }
    )
}
