#![recursion_limit = "128"]
extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{
    parse_macro_input, parse_quote, Data, DeriveInput, Fields, GenericParam, Generics, Ident, Index,
};

#[proc_macro_derive(FromHashMap)]
pub fn from_hashmap(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let idents: Vec<Ident> = match input.data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let mut idents: Vec<Ident> = Vec::new();
                for ref field in fields.named.iter() {
                    match &field.ident {
                        &Some(ref ident) => idents.push(ident.clone()),
                        &None => panic!("Your struct is missing a field identity!"),
                    }
                }
                idents
            }
            Fields::Unnamed(_) => unimplemented!(),
            Fields::Unit => unimplemented!(),
        },
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    };

    let mut keys: Vec<String> = Vec::new();
    for ident in idents.iter() {
        keys.push(ident.to_string());
    }

    let name = input.ident;

    let generics = add_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = quote! {
        fn parse_pair<T>(v: &str) -> T where T : ::std::str::FromStr {
            let res = v.parse::<T>();
            match res {
                Ok(val) => val,
                Err(_) => panic!(format!("Unable to convert given input into required type: {}", v)),
            }
        }

        impl #impl_generics FromHashMap<#name> for #name #ty_generics #where_clause {
            fn from_hashmap(mut hm: ::std::collections::HashMap<String, String>) -> #name {
                let mut settings = #name::default();

                #(
                    match hm.entry(String::from(#keys)) {
                        ::std::collections::hash_map::Entry::Occupied(ent) => {
                            settings.#idents = parse_pair(ent.get().as_str());
                        },
                        ::std::collections::hash_map::Entry::Vacant(_) => (),
                    }
                )*

                settings
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}

fn add_trait_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(heapsize::FromHashMap));
        }
    }
    generics
}
