#![recursion_limit = "1024"]
extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{
    parse_macro_input, parse_quote, Data, DeriveInput, Fields, GenericParam, Generics, Ident, Index,
};

use value::*;

#[proc_macro_derive(FromValue)]
pub fn derive_from_value(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
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

    let mut keys2: Vec<String> = Vec::new();
    for ident in idents.iter() {
        keys2.push(ident.to_string());
    }

    let mut idents2 = Vec::new();
    for ident in idents.iter() {
        idents2.push(ident.clone());
    }

    let name = input.ident;

    let generics = add_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics FromValue<#name> for value::Value #ty_generics #where_clause {

            fn fromValue(self) -> #name {
                let mut settings = #name::default();
                let mut hm: ::std::collections::HashMap<::std::string::String, value::Value> = self.fromValue();

                #(
                    match hm.entry(String::from(#keys)) {
                        ::std::collections::hash_map::Entry::Occupied(ent) => {
                            let v = ent.remove();
                            settings.#idents = v.fromValue();
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
            type_param.bounds.push(parse_quote!(heapsize::FromValue));
        }
    }
    generics
}

#[proc_macro_derive(ToValue)]
pub fn derive_to_value(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
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

    let mut keys2: Vec<String> = Vec::new();
    for ident in idents.iter() {
        keys2.push(ident.to_string());
    }

    let mut idents2 = Vec::new();
    for ident in idents.iter() {
        idents2.push(ident.clone());
    }

    let name = input.ident;

    let generics = add_trait_bounds_2(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics ToValue<#name> for #name #ty_generics #where_clause {

            fn toValue(self) -> value::Value {
                let mut hm: ::std::collections::HashMap<::std::string::String, value::Value> = ::std::collections::HashMap::new();

                #(
                    hm.insert(
                        #keys2.to_string(),
                        self.#idents2.toValue(),
                        );
                )*

                hm.toValue()
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}

fn add_trait_bounds_2(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(heapsize::ToValue));
        }
    }
    generics
}
