#![recursion_limit = "128"]

extern crate proc_macro;

use proc_macro::TokenStream;
use syn::Ident;
use syn::{parse_macro_input, DeriveInput};
use quote::quote;

#[proc_macro_derive(TryFromGremlinMap)]
pub fn from_map(input: TokenStream) -> TokenStream {
    // Parse the string representation into a syntax tree
    let ast = parse_macro_input!(input as DeriveInput);

    // create a vector containing the names of all fields on the struct
    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
        ..
    }) = ast.data
    {
        named
    } else {
        unimplemented!()
    };
    println!("got fields: {:?}", fields);

    let idents: Vec<Ident> = 
        fields.into_iter().map(|field| {
            match &field.ident {
                Some(ident) => {
                    ident.to_owned()
                },
                None => unimplemented!()
            }
        }).collect();

    let idents_types = 
        fields.into_iter().map(|field| {
            let ty = &field.ty;
            quote!{ #ty }
        });
    
    println!("These are the idents: {:?}", idents);
    // the vector of idents.
    let mut keys = Vec::new();
    for ident in idents.iter() {
        keys.push(ident.to_string());
    };

    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let tokens = quote! {
        /// Attempts to convert the given &str into a T, panicing if it's not successful
        fn parse_pair<'a, T: gremlin_client::conversion::BorrowFromGValue>(v: &'a gremlin_client::GValue) -> std::result::Result<&'a T, gremlin_client::GremlinError>{
            v.get::<T>()
        }

        impl #impl_generics TryFromGremlinMap<#name> for #name #ty_generics #where_clause {
            fn try_from_gremlin_map(mut hm: ::std::collections::HashMap<String, gremlin_client::GValue>) -> std::result::Result<#name, gremlin_client::GremlinError> {
            //fn from_hashmap(mut hm: ::std::collections::HashMap<String, gremlin_client::GValue>) -> #name {
                // start with the default implementation
                let mut settings = #name::default();
                #(
                    match hm.entry(String::from(#keys)) {
                        ::std::collections::hash_map::Entry::Occupied(occ_ent) => {
                            // set the corresponding struct field to the value in
                            // the corresponding hashmap if it contains it
                            //settings.#idents = parse_pair::<#idents_types>(occ_ent.get()).unwrap().clone();
                            match parse_pair::<#idents_types>(occ_ent.get()){
                                Ok(v) => settings.#idents = v.clone(),
                                Err(err) => return std::result::Result::Err(err)
                            };
                        },
                        ::std::collections::hash_map::Entry::Vacant(_) => return std::result::Result::Err(gremlin_client::GremlinError::MapError("Did not find key".to_string())),
                    }
                )*

                // return the modified struct
                std::result::Result::Ok(settings)
            }
        }
    };
    tokens.into()
}