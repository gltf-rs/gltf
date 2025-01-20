//! Macros used by the `gltf` crate.

// Adapted from `validator_derive` (https://github.com/Keats/validator).
//
// See LICENSE for details.

#![recursion_limit = "128"]

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_quote, DeriveInput};

/// Provided `struct` attributes.
enum StructAttribute {
    /// Identifies an indexable data structure.
    ///
    /// Data structures marked with `#[gltf(indexed)]` will have an
    /// extra `fn index(&self) -> usize` function defined in their
    /// generated reader type.
    Indexed,

    /// A hook for extra validation steps applied to the whole struct.
    Validate(syn::Ident),
}

/// Provided attributes for named `struct` fields.
enum FieldAttribute {
    /// Provides a field with a default value produced by an expression.
    Default(Option<syn::Expr>),

    /// Identifies a field belonging to a particular extension.
    ///
    /// Fields marked with `#[gltf(extension = "EXT_foo")]` are grouped together and
    /// (de)serialized in a separate extension JSON object.
    Extension(syn::Ident),

    /// A hook for extra validation steps applied to a single field.
    Validate(syn::Ident),
}

impl syn::parse::Parse for StructAttribute {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::parse::Result<Self> {
        let tag = input.parse::<syn::Ident>()?;
        match tag.to_string().as_str() {
            "indexed" => Ok(Self::Indexed),
            "validate" => {
                let _eq = input.parse::<syn::Token![=]>()?;
                let literal = input.parse::<syn::LitStr>()?;
                let ident = syn::Ident::new(&literal.value(), tag.span());
                Ok(Self::Validate(ident))
            }
            unrecognized => {
                panic!("gltf({unrecognized}) is not a recognized `struct` attribute")
            }
        }
    }
}

impl syn::parse::Parse for FieldAttribute {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::parse::Result<Self> {
        let tag = input.parse::<syn::Ident>()?;
        match tag.to_string().as_str() {
            "default" => {
                if input.is_empty() {
                    Ok(Self::Default(None))
                } else {
                    let _eq = input.parse::<syn::Token![=]>();
                    let expr = input.parse::<syn::Expr>()?;
                    Ok(Self::Default(Some(expr)))
                }
            }
            "extension" => {
                let _eq = input.parse::<syn::Token![=]>()?;
                let literal = input.parse::<syn::LitStr>()?;
                let ident = syn::Ident::new(&literal.value(), tag.span());
                Ok(Self::Extension(ident))
            }
            "validate" => {
                let _eq = input.parse::<syn::Token![=]>()?;
                let literal = input.parse::<syn::LitStr>()?;
                let ident = syn::Ident::new(&literal.value(), tag.span());
                Ok(Self::Validate(ident))
            }
            unrecognized => {
                panic!("gltf({unrecognized}) is not a recognized named `struct` field attribute")
            }
        }
    }
}

/// Implements the `Default` trait.
///
/// This macro is similar to the built-in `#[derive(Default)]` but allows default values for fields
/// to be defined inline using the `#[gltf(default = ...)]` attribute.
///
/// # Basic usage
///
/// Declaration
///
/// ```rust
/// #[derive(gltf_derive::Default)]
/// struct Example {
///     #[gltf(default = 123)]
///     pub foo: i32,
///     pub bar: Option<i32>,
/// }
///
/// let example: Example = Default::default();
/// assert_eq!(example.foo, 123);
/// assert_eq!(example.bar, None);
/// ```
#[proc_macro_derive(Default, attributes(gltf))]
pub fn derive_default(input: TokenStream) -> TokenStream {
    expand_default(&syn::parse_macro_input!(input as DeriveInput)).into()
}

fn expand_default(ast: &DeriveInput) -> TokenStream2 {
    let ident = &ast.ident;
    match ast.data {
        syn::Data::Struct(ref data) => impl_default_for_struct(ident, data),
        _ => panic!("#[derive(Default)] only works on `struct` declarations"),
    }
}

fn impl_default_for_struct(ident: &syn::Ident, data: &syn::DataStruct) -> TokenStream2 {
    let mut per_field_idents = Vec::new();
    let mut per_field_defaults = Vec::new();
    for field in &data.fields {
        let mut default = None;
        for attr in &field.attrs {
            if attr.path().is_ident("gltf") {
                let parsed_attribute = attr
                    .parse_args::<FieldAttribute>()
                    .expect("failed to parse attribute");
                if let FieldAttribute::Default(Some(expr)) = parsed_attribute {
                    default = Some(quote! { #expr });
                }
            }
        }

        per_field_idents.push(&field.ident);
        if let Some(expr) = default {
            per_field_defaults.push(quote! { #expr });
        } else {
            let type_ = &field.ty;
            per_field_defaults.push(quote! { <#type_>::default() });
        }
    }

    quote! {
        impl Default for #ident {
            fn default() -> Self {
                Self {
                    #(
                        #per_field_idents: #per_field_defaults,
                    )*
                }
            }
        }
    }
}

/// Implements the `Stub` trait.
///
/// # Basic usage
///
/// Declaration
///
/// ```rust
/// #[derive(gltf_derive::Stub)]
/// struct Example {
///     pub foo: i32,
///     pub bar: Option<i32>,
/// }
///
/// let example: Example = Stub::stub();
/// assert_eq!(example.foo, 0);
/// assert_eq!(example.bar, None);
/// ```
#[proc_macro_derive(Stub, attributes(gltf))]
pub fn derive_stub(input: TokenStream) -> TokenStream {
    expand_stub(&syn::parse_macro_input!(input as DeriveInput)).into()
}

fn expand_stub(ast: &DeriveInput) -> TokenStream2 {
    let ident = &ast.ident;
    match ast.data {
        syn::Data::Struct(ref data) => impl_stub_for_struct(ident, data),
        _ => panic!("#[derive(Stub)] only works on `struct` declarations"),
    }
}

fn impl_stub_for_struct(ident: &syn::Ident, data: &syn::DataStruct) -> TokenStream2 {
    let mut per_field_idents = Vec::new();
    let mut per_field_stubs = Vec::new();
    for field in &data.fields {
        let mut default = None;
        for attr in &field.attrs {
            if attr.path().is_ident("gltf") {
                let parsed_attribute = attr
                    .parse_args::<FieldAttribute>()
                    .expect("failed to parse attribute");
                if let FieldAttribute::Default(Some(expr)) = parsed_attribute {
                    default = Some(quote! { #expr });
                } else {
                    default = Some(quote! { Default::default() });
                }
            }
        }

        per_field_idents.push(&field.ident);
        if let Some(expr) = default {
            per_field_stubs.push(quote! { #expr });
        } else {
            let type_ = &field.ty;
            per_field_stubs.push(quote! { <#type_>::stub() });
        }
    }

    quote! {
        impl crate::Stub for #ident {
            fn stub() -> Self {
                Self {
                    #(
                        #per_field_idents: #per_field_stubs,
                    )*
                }
            }
        }
    }
}

/// Implements the `Wrap` trait for a data structure and provides an associated reader type.
///
/// # Basic usage
///
/// Declaration:
///
/// ```rust,no_run
/// # pub type Root = ();
/// #
/// # pub struct Index<T>(std::marker::PhantomData<T>);
/// #
/// # pub trait Wrap<'a> {
/// #     type Wrapped;
/// #     fn wrap(&'a self, root: &'a Root) -> Self::Wrapped;
/// #     fn wrap_indexed(&'a self, root: &'a Root, index: usize) -> Self::Wrapped {
/// #         let _ = index;
/// #         self.wrap(root)
/// #     }
/// # }
/// #
/// # impl<'a, T> Wrap<'a> for Index<T> {
/// #     type Wrapped = T;
/// #     fn wrap(&'a self, _root: &'a Root) -> Self::Wrapped { unimplemented!() }
/// # }
/// #
/// # impl<'a> Wrap<'a> for i32 {
/// #     type Wrapped = i32;
/// #     fn wrap(&'a self, _root: &'a Root) -> Self::Wrapped { unimplemented!() }
/// # }
/// #
/// # fn main() {}
/// #
/// # type Foo = i32;
/// #
/// /// Object documentation.
/// #[derive(gltf_derive::Wrap)]
/// struct Object {
///     /// Documentation for field foo.
///     pub foo: Index<Foo>,
///     /// Documentation for field bar.
///     pub bar: i32,
/// }
///```
///
/// Generated wrap implementation:
///
/// ```rust,no_run
/// # type Object = ();
/// # type Root = ();
/// # trait Wrap<'a> {
/// #     type Wrapped;
/// #     fn wrap(&'a self, root: &'a Root) -> Self::Wrapped;
/// #     fn wrap_indexed(&'a self, root: &'a Root, index: usize) -> Self::Wrapped {
/// #         let _ = index;
/// #         self.wrap(root)
/// #     }
/// # }
/// #
/// #[doc = "Object documentation."]
/// #[derive(Clone, Copy)]
/// struct ObjectReader<'a>(&'a Object, &'a Root, usize);
///
/// impl<'a> Wrap<'a> for Object {
///     // New type generated by this macro—see below.
///     type Wrapped = ObjectReader<'a>;
///
///     fn wrap(&'a self, root: &'a Root) -> Self::Wrapped {
///         ObjectReader(self, root, !0)
///     }
/// }
/// ```
///
/// Generated reader type:
///
/// ```rust,no_run
/// # type Root = ();
/// #
/// # trait Wrap<'a> {
/// #     type Wrapped;
/// #     fn wrap(&'a self, root: &'a Root) -> Self::Wrapped;
/// #     fn wrap_indexed(&'a self, root: &'a Root, index: usize) -> Self::Wrapped {
/// #         let _ = index;
/// #         self.wrap(root)
/// #     }
/// # }
/// #
/// # pub struct Index<T>(std::marker::PhantomData<T>);
/// #
/// # impl<'a, T> Wrap<'a> for Index<T> {
/// #     type Wrapped = T;
/// #     fn wrap(&'a self, _root: &'a Root) -> Self::Wrapped { unimplemented!() }
/// # }
/// #
/// # impl<'a> Wrap<'a> for i32 {
/// #     type Wrapped = i32;
/// #     fn wrap(&'a self, _root: &'a Root) -> Self::Wrapped { unimplemented!() }
/// # }
/// #
/// # type Foo = i32;
/// #
/// # struct Object {
/// #    pub foo: Index<Foo>,
/// #    pub bar: i32,
/// # }
/// #
/// #[doc = "Object documentation."]
/// #[derive(Clone, Copy)]
/// struct ObjectReader<'a>(&'a Object, &'a Root, usize);
///
/// impl<'a> ObjectReader<'a> {
///     #[doc = "Documentation for field foo."]
///     pub fn foo(&self) -> <Index<Foo> as Wrap<'a>>::Wrapped {
///         self.0.foo.wrap(self.1)
///     }
///
///     #[doc = "Documentation for field bar."]
///     pub fn bar(&self) -> <i32 as Wrap<'a>>::Wrapped {
///         self.0.bar.wrap(self.1)
///     }
/// }
/// ```
///
/// # With indexed attribute
///
/// If the type is marked with `#[gltf(indexed)]` then the `wrap_indexed` function is
/// implemented and the reader type gains an additional `index` function:
///
/// ```rust,no_run
/// # pub type Root = ();
/// #
/// # pub struct Index<T>(std::marker::PhantomData<T>);
/// #
/// # pub trait Wrap<'a> {
/// #     type Wrapped;
/// #     fn wrap(&'a self, root: &'a Root) -> Self::Wrapped;
/// #     fn wrap_indexed(&'a self, root: &'a Root, index: usize) -> Self::Wrapped {
/// #         let _ = index;
/// #         self.wrap(root)
/// #     }
/// # }
/// #
/// # impl<'a, T> Wrap<'a> for Index<T>
/// # where T: 'a + Wrap<'a>,
/// # {
/// #     type Wrapped = <T as Wrap<'a>>::Wrapped;
/// #     fn wrap(&'a self, _root: &'a Root) -> Self::Wrapped {
/// #         unimplemented!()
/// #     }
/// # }
/// #
/// # impl<'a> Wrap<'a> for i32 {
/// #     type Wrapped = Self;
/// #     fn wrap(&'a self, _root: &'a Root) -> Self::Wrapped {
/// #         unimplemented!()
/// #     }
/// # }
/// #
/// # fn main() {}
/// #
/// # type Foo = i32;
/// #
/// /// Object documentation.
/// #[derive(gltf_derive::Wrap)]
/// struct Object {
///     /// Documentation for field foo.
///     pub foo: Index<Foo>,
///     /// Documentation for field bar.
///     pub bar: i32,
/// }
///```
/// Generated `Wrap` implementation:
///
/// ```rust,no_run
/// # pub type Root = ();
/// # pub trait Wrap<'a> {
/// #     type Wrapped;
/// #     fn wrap(&'a self, root: &'a Root) -> Self::Wrapped;
/// #     fn wrap_indexed(&'a self, root: &'a Root, index: usize) -> Self::Wrapped {
/// #         let _ = index;
/// #         self.wrap(root)
/// #     }
/// # }
/// #
/// # fn main() {}
/// #
/// # type Object = ();
/// #
/// pub struct ObjectReader<'a>(&'a Object, &'a Root, usize);
///
/// impl<'a> Wrap<'a> for Object {
///     // New type generated by this macro—see below.
///     type Wrapped = ObjectReader<'a>;
///
///     fn wrap(&'a self, root: &'a Root) -> Self::Wrapped {
///         ObjectReader(self, root, !0)
///     }
///
///     fn wrap_indexed(&'a self, root: &'a Root, index: usize) -> Self::Wrapped {
///         ObjectReader(self, root, index)
///     }
/// }
/// ```
///
/// Generated additional `index` function:
///
/// ```rust,no_run
/// # type Object = ();
/// # type Root = ();
/// # struct ObjectReader<'a>(&'a Object, &'a Root, usize);
/// impl<'a> ObjectReader<'a> {
///     /// The index of this object in its parent container.
///     pub fn index(&self) -> usize {
///         self.2
///     }
/// }
/// ```
///
/// The `wrap_indexed` function is called when `Vec<T>` is iterated over using
/// a reader. Refer to the implementation of `Wrap` for `Vec<T>` in the main `gltf`
/// crate for clarity.
#[proc_macro_derive(Wrap, attributes(gltf))]
pub fn derive_wrap(input: TokenStream) -> TokenStream {
    expand_wrap(&syn::parse_macro_input!(input as DeriveInput)).into()
}

fn doc_attribute(attributes: &[syn::Attribute]) -> TokenStream2 {
    for attribute in attributes {
        if let syn::Meta::NameValue(ref name_value) = attribute.meta {
            if name_value.path.is_ident("doc") {
                return quote! { #attribute };
            }
        }
    }

    quote! {
        #[doc = "Missing documentation"]
    }
}

fn wrap_indexed(attributes: &[syn::Attribute]) -> TokenStream2 {
    for attribute in attributes {
        if attribute.path().is_ident("gltf") {
            let parsed_attribute = attribute
                .parse_args::<StructAttribute>()
                .expect("failed to parse `struct` field attribute");
            match parsed_attribute {
                StructAttribute::Indexed => {
                    return quote! {
                        #[doc = "The index of this item in its parent container."]
                        pub fn index(&self) -> usize {
                            self.2
                        }
                    }
                }
                StructAttribute::Validate(_) => {}
            }
        }
    }

    quote! {}
}

fn make_reader_ident(ident: &syn::Ident) -> syn::Ident {
    syn::Ident::new(&format!("{ident}Reader"), ident.span())
}

fn impl_wrap_for_enum(ident: &syn::Ident, _data: &syn::DataEnum) -> TokenStream2 {
    quote! {
        impl<'a> crate::Wrap<'a> for #ident {
            type Wrapped = Self;

            fn wrap(&'a self, _root: &'a crate::Root) -> Self::Wrapped {
                self.clone()
            }
        }
    }
}

fn impl_wrap_for_struct(
    ident: &syn::Ident,
    data: &syn::DataStruct,
    attributes: &[syn::Attribute],
) -> TokenStream2 {
    let fields = &data.fields;
    let reader = make_reader_ident(ident);
    let index_fn = wrap_indexed(attributes);
    let docs = doc_attribute(attributes);
    let per_field_functions = fields
        .iter()
        .map(|f| {
            (
                f.ident.as_ref().unwrap(),
                f.ty.clone(),
                doc_attribute(&f.attrs),
            )
        })
        .map(|(field_ident, field_type, docs)| {
            quote! {
                #docs
                pub fn #field_ident(&self) -> <#field_type as crate::Wrap>::Wrapped {
                    use crate::Wrap;
                    self.1.#field_ident.wrap(self.0)
                }
            }
        })
        .collect::<Vec<_>>();

    quote! {
        #docs
        pub struct #reader<'a>(&'a crate::Root, &'a #ident, usize);

        impl<'a> #reader<'a> {
            #index_fn

            #(#per_field_functions)*
        }

        impl<'a> crate::Wrap<'a> for #ident {
            type Wrapped = #reader<'a>;

            fn wrap(&'a self, root: &'a crate::Root) -> Self::Wrapped {
                #reader(root, self, !0)
            }

            fn wrap_indexed(&'a self, root: &'a crate::Root, index: usize) -> Self::Wrapped {
                #reader(root, self, index)
            }
        }
    }
}

fn expand_wrap(ast: &DeriveInput) -> TokenStream2 {
    let ident = &ast.ident;
    match ast.data {
        syn::Data::Struct(ref data) => impl_wrap_for_struct(ident, data, &ast.attrs),
        syn::Data::Enum(ref data) => impl_wrap_for_enum(ident, data),
        _ => panic!("#[derive(Wrap)] only works on `struct` and `enum` declarations"),
    }
}

/// Implements the `Validate` trait for a `struct` with named fields.
///
/// For data structures, the generated code will call `Validate::validate` on each field
/// with the camel-case name of the field appended to the JSON path builder.
///
/// # Basic usage
///
/// Declaration:
///
/// ```rust,no_run
/// # #[derive(Clone, Copy)]
/// # pub struct Path;
/// #
/// # impl Path {
/// #     pub fn field(self, _: &str) -> Self {
/// #         self
/// #     }
/// # }
/// # pub type Root = ();
/// #
/// # pub mod validation {
/// #     use super::{Path, Root};
/// #
/// #     pub type Error = ();
/// #
/// #     pub trait Validate {
/// #         fn validate<P, R>(&self, _root: &Root, _path: P, _report: &mut R)
/// #         where
/// #             P: Fn() -> Path,
/// #             R: FnMut(&dyn Fn() -> Path, Error),
/// #         {
/// #         }
/// #     }
/// #
/// #     impl Validate for i32 {}
/// # }
/// #
/// # #[derive(gltf_derive::Validate)]
/// # struct FooBar {}
/// #
/// # #[derive(gltf_derive::Validate)]
/// # struct Baz {}
/// #
/// # fn main() {}
/// #
/// #[derive(gltf_derive::Validate)]
/// struct Object {
///     pub foo_bar: FooBar,
///     pub baz: Baz,
/// }
/// ```
///
/// Generated code:
///
/// ```rust,no_run
/// # pub type Root = ();
/// #
/// # #[derive(Clone, Copy)]
/// # pub struct Path;
/// #
/// # impl Path {
/// #     pub fn field(self, _: &str) -> Self {
/// #         self
/// #     }
/// # }
/// #
/// # pub mod validation {
/// #     use super::{Path, Root};
/// #
/// #     pub type Error = ();
/// #
/// #     pub trait Validate {
/// #         fn validate<P, R>(&self, _root: &Root, _path: P, _report: &mut R)
/// #         where
/// #             P: Fn() -> Path,
/// #             R: FnMut(&dyn Fn() -> Path, Error),
/// #         {
/// #         }
/// #     }
/// #
/// #     impl Validate for i32 {}
/// # }
/// #
/// # #[derive(gltf_derive::Validate)]
/// # struct FooBar {}
/// #
/// # #[derive(gltf_derive::Validate)]
/// # struct Baz {}
/// #
/// # struct Object {
/// #     pub foo_bar: FooBar,
/// #     pub baz: Baz,
/// # }
/// #
/// # fn main() {}
/// #
/// # use validation::{Error, Validate};
/// impl Validate for Object {
///     fn validate<P, R>(&self, root: &Root, path: P, report: &mut R)
///     where
///         P: Fn() -> Path,
///         R: FnMut(&dyn Fn() -> Path, Error),
///     {
///         self.foo_bar.validate(root, || path().field("fooBar"), report);
///         self.baz.validate(root, || path().field("baz"), report);
///     }
/// }
/// ```
///
/// # Hooks
///
/// In addition to the standard per field code generation, ad hoc validation can be inserted using
/// the `gltf(validate = "...")` attribute. This attribute can be applied to structs and fields.
///
/// Declaration:
///
/// ```rust,no_run
/// # #[derive(Clone, Copy)]
/// # pub struct Path;
/// #
/// # impl Path {
/// #     pub fn field(self, _: &str) -> Self {
/// #         self
/// #     }
/// # }
/// # pub type Root = ();
/// #
/// # pub mod validation {
/// #     use super::{Path, Root};
/// #
/// #     pub enum Error {
/// #         Invalid,
/// #         Missing,
/// #     }
/// #
/// #     pub trait Validate {
/// #         fn validate<P, R>(&self, _root: &Root, _path: P, _report: &mut R)
/// #         where
/// #             P: Fn() -> Path,
/// #             R: FnMut(&dyn Fn() -> Path, Error),
/// #         {
/// #         }
/// #     }
/// #
/// #     impl Validate for i32 {}
/// # }
/// #
/// # use validation::{Error, Validate};
/// #
/// # impl Validate for Option<i32> {
/// #     fn validate<P, R>(&self, _root: &Root, _path: P, _report: &mut R)
/// #     where
/// #         P: Fn() -> Path,
/// #         R: FnMut(&dyn Fn() -> Path, Error),
/// #     {
/// #     }
/// # }
/// #
/// # #[derive(gltf_derive::Validate)]
/// # struct Foo {}
/// #
/// # fn main() {}
/// #
/// #[derive(gltf_derive::Validate)]
/// #[gltf(validate = "validate_example_struct")]
/// struct ExampleStruct {
///     #[gltf(validate = "validate_example_field")]
///     pub example_field: i32,
///     pub regular_field: Option<i32>,
/// }
///
/// fn validate_example_struct<P, R>(example: &ExampleStruct, _root: &Root, path: P, report: &mut R)
/// where
///     P: Fn() -> Path,
///     R: FnMut(&dyn Fn() -> Path, Error),
/// {
///     if example.regular_field.is_none() {
///         report(&|| path().field("regularField"), Error::Missing);
///     }
/// }
///
/// fn validate_example_field<P, R>(example: &i32, _root: &Root, path: P, report: &mut R)
/// where
///     P: Fn() -> Path,
///     R: FnMut(&dyn Fn() -> Path, Error),
/// {
///     if *example != 42 {
///         report(&path, Error::Invalid);
///     }
/// }
/// ```
///
/// Generated code:
///
/// ```rust,no_run
/// # #[derive(Clone, Copy)]
/// # pub struct Path;
/// #
/// # impl Path {
/// #     pub fn field(self, _: &str) -> Self {
/// #         self
/// #     }
/// # }
/// # pub type Root = ();
/// #
/// # pub mod validation {
/// #     use super::{Path, Root};
/// #
/// #     pub enum Error {
/// #         Invalid,
/// #         Missing,
/// #     }
/// #
/// #     pub trait Validate {
/// #         fn validate<P, R>(&self, _root: &Root, _path: P, _report: &mut R)
/// #         where
/// #             P: Fn() -> Path,
/// #             R: FnMut(&dyn Fn() -> Path, Error),
/// #         {
/// #         }
/// #     }
/// #
/// #     impl Validate for i32 {}
/// # }
/// #
/// # #[derive(gltf_derive::Validate)]
/// # struct Foo {}
/// #
/// # fn main() {}
/// #
/// # use validation::{Error, Validate};
/// #
/// # struct ExampleStruct {
/// #     pub example_field: i32,
/// #     pub regular_field: Option<i32>,
/// # }
/// #
/// # impl Validate for Option<i32> {
/// #     fn validate<P, R>(&self, _root: &Root, _path: P, _report: &mut R)
/// #     where
/// #         P: Fn() -> Path,
/// #         R: FnMut(&dyn Fn() -> Path, Error),
/// #     {
/// #     }
/// # }
/// #
/// # fn validate_example_struct<P, R>(example: &ExampleStruct, _root: &Root, _path: P, _report: &mut R)
/// # where
/// #     P: Fn() -> Path,
/// #     R: FnMut(&dyn Fn() -> Path, Error),
/// # {
/// # }
/// #
/// # fn validate_example_field<P, R>(example: &i32, _root: &Root, _path: P, _report: &mut R)
/// # where
/// #     P: Fn() -> Path,
/// #     R: FnMut(&dyn Fn() -> Path, Error),
/// # {
/// # }
/// #
/// impl Validate for ExampleStruct {
///     fn validate<P, R>(&self, root: &Root, path: P, report: &mut R)
///     where
///         P: Fn() -> Path,
///         R: FnMut(&dyn Fn() -> Path, Error),
///     {
///         validate_example_struct(self, root, &path, report);
///
///         self.example_field.validate(root, || path().field("exampleField"), report);
///         validate_example_field(&self.example_field, root, || path().field("exampleField"), report);
///
///         self.regular_field.validate(root, || path().field("regularField"), report);
///     }
/// }
/// ```
#[proc_macro_derive(Validate, attributes(gltf))]
pub fn derive_validate(input: TokenStream) -> TokenStream {
    expand_validate(&syn::parse_macro_input!(input as DeriveInput)).into()
}

fn expand_validate(ast: &DeriveInput) -> TokenStream2 {
    let mut validate_hook = quote! {};
    for attr in &ast.attrs {
        if attr.path().is_ident("gltf") {
            let parsed_attr = attr
                .parse_args::<StructAttribute>()
                .expect("failed to parse attribute");
            if let StructAttribute::Validate(hook_ident) = parsed_attr {
                validate_hook = quote! {
                    #hook_ident(self, _root, _path, _report);
                };
            }
        }
    }

    let fields = match ast.data {
        syn::Data::Struct(ref data_struct) => &data_struct.fields,
        _ => panic!("#[derive(Validate)] only works on `struct`s"),
    };
    let ident = &ast.ident;
    let validations = fields
        .iter()
        .map(|f| {
            use inflections::Inflect;
            let ident = f.ident.as_ref().unwrap();
            let field = ident.to_string().to_camel_case();

            let mut validate_hook = quote! {};
            for attr in &f.attrs {
                if attr.path().is_ident("gltf") {
                    let parsed_attr = attr
                        .parse_args::<FieldAttribute>()
                        .expect("failed to parse attribute");
                    if let FieldAttribute::Validate(hook_ident) = parsed_attr {
                        validate_hook = quote! {
                            #hook_ident(&self.#ident, _root, || _path().field(#field), _report);
                        };
                    }
                }
            }

            quote!(
                #validate_hook

                self.#ident.validate(
                    _root,
                    || _path().field(#field),
                    _report,
                )
            )
        })
        .collect::<Vec<_>>();
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
                R: FnMut(&dyn Fn() -> crate::Path, crate::validation::Error),
            {
                #(
                    #validations;
                )*

                #validate_hook
            }
        }
    )
}

enum Generator {
    Deserialize,
    Serialize,
}

/// Extension of `serde_derive::Deserialize` for glTF objects.
///
/// This macro allows fields for glTF extensions to be declared alongside standard glTF fields.
/// Attributes from `serde_derive` are compatible with this macro.
///
/// The generated code identifies fields marked with the `#[gltf(extension = "...")]` attribute
/// and deserializes them separately from the `"extensions"` object in the glTF JSON. It also
/// applies some common serde attributes automatically.
///
/// # Basic usage
///
/// Declaration:
///
/// ```rust,no_run
/// # type UnrecognizedExtensions = serde_json::Map<String, serde_json::Value>;
/// #
/// #[derive(serde_derive::Deserialize)]
/// struct ObjectExtensionFoo(pub i32);
///
/// #[derive(gltf_derive::Deserialize)]
/// struct Object {
///     // Note: extension field types must appear within an `Option` type.
///     #[gltf(extension = "EXT_object_foo")]
///     pub foo: Option<ObjectExtensionFoo>,
///     pub bar: i32,
///     pub unrecognized_extensions: UnrecognizedExtensions,
/// }
/// ```
///
/// Generated code:
///
/// ```rust,no_run
/// # #[derive(serde_derive::Deserialize)]
/// # struct ObjectExtensionFoo(pub i32);
/// #
/// # struct Object {
/// #     pub foo: Option<ObjectExtensionFoo>,
/// #     pub bar: i32,
/// # }
/// #
/// #[allow(non_snake_case)]
/// impl<'de> serde::Deserialize<'de> for Object {
///     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
///     where
///         D: serde::de::Deserializer<'de>,
///     {
///         #[derive(Default, serde_derive::Deserialize)]
///         struct Extensions {
///             #[serde(default)]
///             EXT_object_foo: Option<ObjectExtensionFoo>,
///         }
///
///         #[derive(serde_derive::Deserialize)]
///         struct Intermediate {
///             #[serde(default)]
///             extensions: Extensions,
///             bar: i32,
///         }
///
///         let intermediate = Intermediate::deserialize(deserializer)?;
///         Ok(Self {
///             foo: intermediate.extensions.EXT_object_foo,
///             bar: intermediate.bar,
///         })
///     }
/// }
/// ```
///
/// # Automatically applied attributes
///
/// `#[serde(default)]` is applied for `bool`, `Option`, and `Vec` types.
///
/// Declaration:
///
/// ```rust,no_run
/// # type UnrecognizedExtensions = serde_json::Map<String, serde_json::Value>;
/// #[derive(gltf_derive::Deserialize)]
/// struct Foo {
///     a: i32,
///     b: bool,
///     c: Option<i32>,
///     d: Vec<i32>,
///     unrecognized_extensions: UnrecognizedExtensions,
/// }
/// ```
///
/// Equivalent code:
///
/// ```rust,no_run
/// # type UnrecognizedExtensions = serde_json::Map<String, serde_json::Value>;
/// #[derive(serde_derive::Deserialize)]
/// struct Foo {
///     a: i32,
///     #[serde(default)]
///     b: bool,
///     #[serde(default)]
///     c: Option<i32>,
///     #[serde(default)]
///     d: Vec<i32>,
///     #[serde(flatten)]
///     unrecognized_extensions: UnrecognizedExtensions,
/// }
/// ```
#[proc_macro_derive(Deserialize, attributes(gltf))]
pub fn derive_deserialize(input: TokenStream) -> TokenStream {
    expand(
        Generator::Deserialize,
        &syn::parse_macro_input!(input as DeriveInput),
    )
    .into()
}

/// Extension of `serde_derive::Serialize` for glTF objects.
///
/// This macro allows fields for glTF extensions to be declared alongside standard glTF fields.
/// Attributes from `serde_derive` are compatible with this macro.
///
/// The generated code identifies fields marked with the `#[gltf(extension = "...")]` attribute
/// and serializes them separately into the `"extensions"` object in the glTF JSON. It also
/// applies some common serde attributes automatically.
///
/// # Basic usage
///
/// Declaration:
///
/// ```rust,no_run
/// # type UnrecognizedExtensions = ::serde_json::Map<String, ::serde_json::Value>;
/// #
/// #[derive(serde_derive::Serialize)]
/// struct ObjectExtensionFoo(i32);
///
/// #[derive(gltf_derive::Serialize)]
/// struct Object {
///     // Note: extension field types must appear within an `Option` type.
///     #[gltf(extension = "EXT_object_foo")]
///     pub foo: Option<ObjectExtensionFoo>,
///     pub bar: i32,
///     pub unrecognized_extensions: UnrecognizedExtensions,
/// }
/// ```
///
/// Generated code:
///
/// ```rust,no_run
/// # type UnrecognizedExtensions = ::serde_json::Map<String, ::serde_json::Value>;
/// #
/// # #[derive(serde_derive::Serialize)]
/// # struct ObjectExtensionFoo(pub i32);
/// #
/// # struct Object {
/// #    pub foo: Option<ObjectExtensionFoo>,
/// #    pub bar: i32,
/// #    pub unrecognized_extensions: UnrecognizedExtensions,
/// # }
/// #
/// #[allow(non_snake_case)]
/// impl serde::Serialize for Object {
///     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
///     where
///         S: serde::ser::Serializer,
///     {
///         #[derive(serde_derive::Serialize)]
///         struct Extensions<'a> {
///             #[serde(skip_serializing_if = "Option::is_none")]
///             EXT_object_foo: &'a Option<ObjectExtensionFoo>,
///             #[serde(flatten)]
///             unrecognized: &'a  UnrecognizedExtensions,
///         }
///
///         impl<'a> Extensions<'a> {
///             fn is_empty(&self) -> bool {
///                 self.EXT_object_foo.is_none() && self.unrecognized.is_empty()
///             }
///         }
///
///         #[derive(serde_derive::Serialize)]
///         struct Intermediate<'a> {
///             #[serde(skip_serializing_if = "Extensions::is_empty")]
///             extensions: Extensions<'a>,
///             bar: &'a i32,
///         }
///
///         let intermediate = Intermediate {
///             bar: &self.bar,
///             extensions: Extensions {
///                 EXT_object_foo: &self.foo,
///                 unrecognized: &self.unrecognized_extensions,
///             },
///         };
///
///         intermediate.serialize(serializer)
///     }
/// }
/// ```
///
/// # Automatically applied attributes
///
/// `#[serde(skip_serializing_if = "...")]` is applied for `bool`, `Option`, and `Vec` types.
///
/// Declaration:
///
/// ```rust,no_run
/// # type UnrecognizedExtensions = serde_json::Map<String, serde_json::Value>;
/// #[derive(gltf_derive::Serialize)]
/// struct Foo {
///     a: i32,
///     b: bool,
///     c: Option<i32>,
///     d: Vec<i32>,
///     unrecognized_extensions: UnrecognizedExtensions,
/// }
/// ```
///
/// Equivalent code:
///
/// ```rust,no_run
/// # type UnrecognizedExtensions = serde_json::Map<String, serde_json::Value>;
/// #[derive(serde_derive::Serialize)]
/// struct Foo {
///     a: i32,
///     #[serde(skip_serializing_if = "std::ops::Not::not")]
///     b: bool,
///     #[serde(skip_serializing_if = "Option::is_none")]
///     c: Option<i32>,
///     #[serde(skip_serializing_if = "Vec::is_empty")]
///     d: Vec<i32>,
///     unrecognized_extensions: UnrecognizedExtensions,
/// }
/// ```
#[proc_macro_derive(Serialize, attributes(gltf, serde))]
pub fn derive_serialize(input: TokenStream) -> TokenStream {
    expand(
        Generator::Serialize,
        &syn::parse_macro_input!(input as DeriveInput),
    )
    .into()
}

fn expand(generator: Generator, ast: &DeriveInput) -> TokenStream2 {
    match ast.data {
        syn::Data::Struct(ref data) => expand_for_struct(generator, &ast.ident, data, &ast.attrs),
        _ => panic!("gltf_derive::Deserialize only works on `struct` declarations"),
    }
}

fn has_type_name(type_: &syn::Type, type_name: &str) -> bool {
    match type_ {
        syn::Type::Path(type_path) => {
            if let Some(segment) = type_path.path.segments.first() {
                segment.ident == type_name
            } else {
                false
            }
        }
        syn::Type::Reference(ref_type) => has_type_name(&ref_type.elem, type_name),
        _ => false,
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SpecialCase {
    /// The `bool` primitive type.
    Bool,
    /// The standard library `Option<T>` type.
    Option,
    /// The standard library `Vec<T>` type.
    Vec,
}

impl SpecialCase {
    pub fn skip_serializing_attribute(self) -> syn::Attribute {
        match self {
            Self::Bool => parse_quote! { #[serde(skip_serializing_if = "is_false")] },
            Self::Option => parse_quote! { #[serde(skip_serializing_if = "Option::is_none")] },
            Self::Vec => parse_quote! { #[serde(skip_serializing_if = "Vec::is_empty")] },
        }
    }
}

fn detect_special_case(type_: &syn::Type) -> Option<SpecialCase> {
    if has_type_name(type_, "bool") {
        Some(SpecialCase::Bool)
    } else if has_type_name(type_, "Option") {
        Some(SpecialCase::Option)
    } else if has_type_name(type_, "Vec") {
        Some(SpecialCase::Vec)
    } else {
        None
    }
}

struct Attributes(pub Vec<syn::Attribute>);

impl quote::ToTokens for Attributes {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        for attribute in &self.0 {
            attribute.to_tokens(tokens);
        }
    }
}

fn expand_for_struct(
    generator: Generator,
    ident: &syn::Ident,
    data: &syn::DataStruct,
    attrs: &[syn::Attribute],
) -> TokenStream2 {
    let serde_attrs = attrs
        .iter()
        .filter(|attr| attr.path().is_ident("serde"))
        .collect::<Vec<_>>();

    let mut ext_attrs = Vec::new();
    let mut ext_idents = Vec::new();
    let mut ext_types = Vec::new();
    let mut ext_renames = Vec::new();

    let mut core_attrs = Vec::new();
    let mut core_idents = Vec::new();
    let mut core_types = Vec::new();

    let mut is_default_fn_bodies = Vec::new();
    let mut default_fn_bodies = Vec::new();

    for field in &data.fields {
        if field
            .ident
            .as_ref()
            .map(|ident| ident == "unrecognized_extensions")
            .unwrap_or_default()
        {
            // Avoid double field.
            continue;
        }

        let mut is_extension = false;
        let mut default_attr = None; // Option<Option<syn::Expr>>
        for attr in &field.attrs {
            if attr.path().is_ident("gltf") {
                let parsed_attribute = attr
                    .parse_args::<FieldAttribute>()
                    .expect("failed to parse attribute");
                match parsed_attribute {
                    FieldAttribute::Default(expr) => {
                        default_attr = Some(expr);
                    }
                    FieldAttribute::Extension(ident) => {
                        is_extension = true;
                        ext_renames.push(ident);
                    }
                    FieldAttribute::Validate(_) => {}
                }
            }
        }

        if is_extension && default_attr.is_some() {
            panic!("#[gltf(default)] cannot be combined with #[gltf(extension)]");
        }

        // We need to filter out any gltf attributes from the next stage of compilation
        // or else they will cease to be recognized and compilation will fail.
        let mut filtered_attrs = field
            .attrs
            .iter()
            .filter(|attr| attr.path().is_ident("serde"))
            .cloned()
            .collect::<Vec<_>>();

        // Insert extra attributes for special cases such as `Option` and `Vec` types.
        if let Some(special_case) = detect_special_case(&field.ty) {
            match generator {
                Generator::Serialize => {
                    filtered_attrs.push(special_case.skip_serializing_attribute());
                }
                Generator::Deserialize => {
                    filtered_attrs.push(parse_quote! { #[serde(default)] });
                }
            }
        }

        if let Some(default) = default_attr {
            match generator {
                Generator::Serialize => {
                    let ident = field.ident.as_ref().unwrap();
                    let type_ = &field.ty;
                    let is_default_fn_name =
                        syn::Ident::new(&format!("{}_is_default", ident), ident.span());
                    let is_default_fn_body = if let Some(expr) = default {
                        quote! {
                            fn #is_default_fn_name(value: &#type_) -> bool {
                                *value == #expr
                            }
                        }
                    } else {
                        // The default case relies on `PartialEq` being implemented.
                        // A special case is required for `Option<T>` and `Vec<T>`
                        // since `T` might not implement the `PartialEq` trait.
                        match detect_special_case(type_) {
                            Some(SpecialCase::Option) => {
                                quote! {
                                    fn #is_default_fn_name(value: &#type_) -> bool {
                                        value.is_none()
                                    }
                                }
                            }
                            Some(SpecialCase::Vec) => {
                                quote! {
                                    fn #is_default_fn_name(value: &#type_) -> bool {
                                        value.is_empty()
                                    }
                                }
                            }
                            _ => {
                                quote! {
                                    fn #is_default_fn_name(value: &#type_) -> bool {
                                        *value == <#type_>::default()
                                    }
                                }
                            }
                        }
                    };
                    is_default_fn_bodies.push(is_default_fn_body);
                    let is_default_fn_lit = syn::LitStr::new(
                        &format!("{is_default_fn_name}"),
                        proc_macro2::Span::call_site(),
                    );
                    filtered_attrs
                        .push(parse_quote! { #[serde(skip_serializing_if = #is_default_fn_lit)] });
                }
                Generator::Deserialize => {
                    let ident = field.ident.as_ref().unwrap();
                    let type_ = &field.ty;
                    let default_fn_name =
                        syn::Ident::new(&format!("{}_default", ident), ident.span());
                    let default_fn_body = if let Some(expr) = default {
                        quote! {
                            fn #default_fn_name() -> #type_ {
                                #expr
                            }
                        }
                    } else {
                        quote! {
                            fn #default_fn_name() -> #type_ {
                                Default::default()
                            }
                        }
                    };
                    default_fn_bodies.push(default_fn_body);
                    let default_fn_lit = syn::LitStr::new(
                        &format!("{default_fn_name}"),
                        proc_macro2::Span::call_site(),
                    );
                    filtered_attrs.push(parse_quote! { #[serde(default = #default_fn_lit)] });
                }
            }
        }

        if is_extension {
            ext_attrs.push(Attributes(filtered_attrs));
            ext_idents.push(&field.ident);
            ext_types.push(&field.ty);
        } else {
            core_attrs.push(Attributes(filtered_attrs));
            core_idents.push(&field.ident);
            core_types.push(&field.ty);
        }
    }

    match generator {
        Generator::Deserialize => quote! {
            #[allow(non_snake_case)]
            impl<'de> ::serde::de::Deserialize<'de> for #ident {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    #[derive(Default, serde_derive::Deserialize)]
                    #(#serde_attrs)*
                    struct Extensions {
                        #(
                            #ext_attrs
                            #ext_renames: #ext_types,
                        )*

                        #[serde(flatten)]
                        unrecognized: ::serde_json::Map<String, ::serde_json::Value>,
                    }

                    #(#default_fn_bodies)*

                    #[derive(serde_derive::Deserialize)]
                    #[serde(rename_all = "camelCase")]
                    #(#serde_attrs)*
                    struct Intermediate {
                        #(
                            #core_attrs
                            #core_idents: #core_types,
                        )*

                        #[serde(default)]
                        extensions: Extensions,
                    }

                    let intermediate = Intermediate::deserialize(deserializer)?;

                    Ok(Self {
                        #(#core_idents: intermediate.#core_idents,)*
                        #(#ext_idents: intermediate.extensions.#ext_renames,)*
                        unrecognized_extensions: intermediate.extensions.unrecognized,
                    })
                }
            }
        },
        Generator::Serialize => quote! {
            #[allow(non_snake_case)]
            impl ::serde::ser::Serialize for #ident {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    #[allow(unused)]
                    #[inline]
                    fn is_false(b: &bool) -> bool {
                        !*b
                    }

                    #[derive(serde_derive::Serialize)]
                    #(#serde_attrs)*
                    struct Extensions<'a> {
                        #(
                            #ext_attrs
                            #ext_renames: &'a #ext_types,
                        )*

                        #[serde(flatten)]
                        unrecognized: &'a ::serde_json::Map<String, ::serde_json::Value>,
                    }

                    impl<'a> Extensions<'a> {
                        fn is_empty(&self) -> bool {
                            #(self.#ext_renames.is_none() &&)*
                            self.unrecognized.is_empty()
                        }
                    }

                    #(#is_default_fn_bodies)*

                    #[derive(serde_derive::Serialize)]
                    #[serde(rename_all = "camelCase")]
                    #(#serde_attrs)*
                    struct Intermediate<'a> {
                        #(
                            #core_attrs
                            #core_idents: &'a #core_types,
                        )*

                        #[serde(skip_serializing_if = "Extensions::is_empty")]
                        extensions: Extensions<'a>,
                    }

                    let intermediate = Intermediate {
                        #(
                            #core_idents: &self.#core_idents,
                        )*
                        extensions: Extensions {
                            #(#ext_renames: &self.#ext_idents,)*
                            unrecognized: &self.unrecognized_extensions,
                        },
                    };

                    intermediate.serialize(serializer)
                }
            }
        },
    }
}

mod tests {
    #[test]
    fn detect_special_cases() {
        use super::{detect_special_case, SpecialCase};
        use syn::parse_quote;

        assert_eq!(
            Some(SpecialCase::Option),
            detect_special_case(&parse_quote! { Option<i32> }),
        );

        assert_eq!(
            Some(SpecialCase::Option),
            detect_special_case(&parse_quote! { Option<Vec<i32>> }),
        );

        assert_eq!(
            Some(SpecialCase::Option),
            detect_special_case(&parse_quote! { Option<(i32, i32)> }),
        );

        assert_eq!(
            Some(SpecialCase::Option),
            detect_special_case(&parse_quote! { &'a Option<i32> }),
        );

        assert_eq!(
            Some(SpecialCase::Vec),
            detect_special_case(&parse_quote! { Vec<Option<i32>> }),
        );

        assert_eq!(
            Some(SpecialCase::Vec),
            detect_special_case(&parse_quote! { Vec<(i32, i32)> }),
        );

        assert_eq!(
            Some(SpecialCase::Vec),
            detect_special_case(&parse_quote! { &'a Vec<i32> }),
        );

        assert_eq!(
            Some(SpecialCase::Bool),
            detect_special_case(&parse_quote! { bool }),
        );

        assert_eq!(
            Some(SpecialCase::Bool),
            detect_special_case(&parse_quote! { &'a bool }),
        );

        assert_eq!(None, detect_special_case(&parse_quote! { i32 }),);
    }
}
