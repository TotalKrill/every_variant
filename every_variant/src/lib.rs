//! Proc macros that can help with generating boilerplate code for parsing structures and enums
//! from topic and payloads from MQTT
extern crate syn;

use quote::*;

extern crate proc_macro;
use proc_macro::TokenStream;

use proc_macro_error::abort;
use syn::{Ident, Item, TypePath};

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
                            abort!(fields, "Cannot have multiple unnamed fields");
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
                        abort!(var, "Does not support Named fields");
                    }
                }
            }
            let (impl_generics, ty_generics, where_clause) = it.generics.split_for_impl();

            let out = quote! {
                impl #impl_generics EveryVariant for #name #ty_generics #where_clause {
                    fn every_variant() -> std::vec::Vec<Self> {
                        let mut vec = std::vec::Vec::new();
                        #( #variant_generators )*
                        vec
                    }

                    fn for_every_variant<F: Fn(&Self)>(closure: F) {
                        let v = Self::every_variant();

                        for elem in &v {
                            closure(&elem);
                        }
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
            };

            let name = &it.ident;

            //let mut member_generator = Vec::new();
            let mut fieldgens = Vec::new();

            for field in &it.fields {
                //println!("field: {:?}", field);
                if let Some(name) = field.ident.clone() {
                    if let syn::Type::Path(path) = field.ty.clone() {
                        let fieldgen = FieldGen { name, ty: path };
                        fieldgens.push(fieldgen);
                    } else {
                        abort!(field, "Ident is missing");
                    }
                } else {
                    abort!(field, "Does not support unnamed fields yet");
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
                let mut ftype = field.ty.clone();
                //println!("type: {}, dbg: {:?}", ftype.to_token_stream(), ftype);

                if let Some(path) = ftype.path.segments.first_mut() {
                    if let syn::PathArguments::AngleBracketed(ref mut args) = &mut path.arguments {
                        args.colon2_token = Some(syn::token::Colon2::default());
                    }
                }

                structgen = quote! {
                    for #fname in #ftype::every_variant() {
                        #structgen
                    }
                };


            }

            let (impl_generics, ty_generics, where_clause) = it.generics.split_for_impl();
            let out = quote! {
                impl #impl_generics EveryVariant for #name #ty_generics #where_clause {
                    fn every_variant() -> std::vec::Vec<Self> {
                        let mut vec = std::vec::Vec::new();
                        #structgen
                        vec
                    }
                }
            };

            //println!("{}", out);
            out.into()
        }
        _ => {
            abort!(item, "Only has an effect on enums and structs");
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
