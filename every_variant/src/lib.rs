#![feature(proc_macro_diagnostic)]

//! Proc macros that can help with generating boilerplate code for parsing structures and enums
//! from topic and payloads from MQTT
extern crate syn;

use quote::*;

extern crate proc_macro;
use proc_macro::TokenStream;

use syn::spanned::Spanned;
use syn::{Attribute, Ident, Item, LitStr, Meta, TypePath};

#[proc_macro_derive(EveryVariant)]
pub fn mqtt_from_inner_payload(item: TokenStream) -> TokenStream {
    let item: syn::Item = syn::parse(item).expect("Failed to parse input item");

    match item {
        Item::Enum(ref it) => {
            //println!("Enum: {}", it.ident);
            let _attrs = &it.attrs;

            let name = &it.ident;

            let variants = &it.variants;

            let mut variant_generators = Vec::new();
            for var in variants {
                let varid = &var.ident;

                match var.fields {
                    syn::Fields::Unnamed(ref fields) => {
                        let f = fields.unnamed.iter();

                        if f.count() > 1 {
                            fields
                                .span()
                                .unstable()
                                .error("Cannot have multiple unnamed fields")
                                .emit();
                        }

                        let field = fields.unnamed.iter().next().unwrap();
                        let field = &field.ty;

                        let variant_gen = quote! {
                            for v in #field::every_variant() {
                                vec.push(Self::#varid(v));
                            }
                        };

                        variant_generators.push(variant_gen);
                        //println!("quote: {:?}", layeridarm.to_string());
                    }
                    syn::Fields::Unit => {
                        // Generate a parsing arm for unit structs
                        let variant_gen = quote! {
                            vec.push(Self::#varid);
                        };
                        variant_generators.push(variant_gen);
                    }
                    _ => {
                        var.span()
                            .unstable()
                            .error("Does not support Named fields")
                            .emit();
                    }
                }
            }
            let (impl_generics, ty_generics, where_clause) = it.generics.split_for_impl();

            let out = quote! {
                impl #impl_generics EveryVariant for #name #ty_generics #where_clause {
                    fn every_variant() -> Vec<Self> {
                        let mut vec = Vec::new();
                        #( #variant_generators )*
                        vec
                    }

                }
            };

            //println!("{}", out);
            out.into()
        }
        Item::Struct(ref it) => {
            struct FieldGen {
                name: Ident,
                ty: TypePath,
                level: usize,
            };

            let name = &it.ident;

            //let mut member_generator = Vec::new();
            let mut fieldgens = Vec::new();

            let mut level = 0;
            for field in &it.fields {
                //println!("field: {:?}", field);
                if let Some(name) = field.ident.clone() {
                    if let syn::Type::Path(path) = field.ty.clone() {
                        let fieldgen = FieldGen {
                            name: name,
                            ty: path,
                            level: level,
                        };
                        level += 1;
                        fieldgens.push(fieldgen);
                    }
                } else {
                    field
                        .span()
                        .unstable()
                        .error("Does not support unnamed fields yet")
                        .emit();
                }
            }

            let names: Vec<Ident> = fieldgens.iter().map(|f| f.name.clone()).collect();
            let mut structgen = quote! {
                let s = Self {
                  #( #names: #names.clone() ),*
                };
                vec.push(s);
            };

            for field in fieldgens.iter().rev() {
                let fname = &field.name;
                let ftype = &field.ty;
                structgen = quote! {
                    for #fname in #ftype::every_variant() {
                        #structgen
                    }
                }
            }

            let (impl_generics, ty_generics, where_clause) = it.generics.split_for_impl();
            let out = quote! {
                impl #impl_generics EveryVariant for #name #ty_generics #where_clause {
                    fn every_variant() -> Vec<Self> {
                        let mut vec = Vec::new();
                        #structgen
                        vec
                    }

                }
            };

            //println!("{}", out);
            out.into()
        }
        _ => {
            item.span()
                .unstable()
                .warning("Only has an effect on enums and structs")
                .emit();
            TokenStream::new()
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
