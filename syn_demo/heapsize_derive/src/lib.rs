/*
 * https://github.com/dtolnay/syn
 * https://docs.rs/syn/0.15.39/syn/struct.DeriveInput.html
 */
extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{
    parse_macro_input, parse_quote, Data, DeriveInput, Fields, GenericParam, Generics, Index,
};

#[proc_macro_derive(HeapSize)]
pub fn derive_heap_size(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    println!("input.attrs.len() = {}", input.attrs.len());
    //println!("input.vis = {:?}", input.vis);
    println!("input.ident = {}", input.ident);

    let name = input.ident;

    let generics = add_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let sum = heap_size_sum(&input.data);

    let expanded = quote! {
        impl #impl_generics heapsize::HeapSize for #name #ty_generics #where_clause {
            fn heap_size_of_children(&self) -> usize {
                #sum
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}

fn add_trait_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(heapsize::HeapSize));
        }
    }
    generics
}

fn heap_size_sum(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let recurse = fields.named.iter().map(|f| {
                    let name = &f.ident;
                    quote_spanned! {
                        f.span() => heapsize::HeapSize::heap_size_of_children(&self.#name)
                    }
                });
                quote! {
                    0 #(+ #recurse)*
                }
            }
            Fields::Unnamed(ref fields) => {
                let recurse = fields.unnamed.iter().enumerate().map(|(i, f)| {
                    let index = Index::from(i);
                    quote_spanned! {
                        f.span() => heapsize::HeapSize::heap_size_of_children(&self.#index)
                    }
                });
                quote! {
                    0 #(+ #recurse)*
                }
            }
            Fields::Unit => quote!(0),
        },
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}
