//! Proc macros that can help with generating boilerplate code for parsing structures and enums
//! from topic and payloads from MQTT
extern crate syn;

use quote::*;

extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};

use proc_macro_error::{abort, proc_macro_error};
use syn::{punctuated::Punctuated, token::Comma, Field, Ident, Item, Type, TypePath};

#[derive(Debug)]
struct StructFieldGen {
    name: Ident,
    ty: TypePath,
}

#[derive(Debug)]
struct AnonStructFieldGen {
    id: Ident,
    ty: TypePath,
}

#[derive(Debug)]
struct EnumFieldGen {
    id: Ident,
    ty: Type,
    name: Option<Ident>,
}

fn do_enum_gen(var_id: &Ident, field_data: &Punctuated<Field, Comma>) -> TokenStream2 {
    let mut field_gen = Vec::new();
    for (idx, field) in field_data.iter().enumerate() {
        field_gen.push(EnumFieldGen {
            id: Ident::new(&format!("v{}", &idx.to_string()), Span::call_site()),
            ty: field.ty.clone(),
            name: field.ident.clone(),
        });
    }

    let mut named_fields = false;

    let names: Vec<TokenStream2> = field_gen
        .iter()
        .map(|field_gen| {
            let field_id = &field_gen.id;
            let field_name = &field_gen.name;
            if let Some(field_name) = field_name {
                named_fields = true;
                quote! {
                    #field_name: #field_id
                }
            } else {
                quote! {
                    #field_id
                }
            }
        })
        .collect();

    let mut enum_gen = if !named_fields {
        quote! {
            let s = Self :: #var_id (
              #( #names.clone() ),*
            );
            vec.push(s);
        }
    } else {
        quote! {
            let s = Self :: #var_id {
              #( #names.clone() ),*
            };
            vec.push(s);
        }
    };

    for field in field_gen.iter().rev() {
        let fname = &field.id;
        let ftype = &field.ty;

        enum_gen = quote! {
            for #fname in <#ftype as EveryVariant>::every_variant() {
                #enum_gen
            }
        };
    }

    let variant_gen = quote! {
        #enum_gen
    };

    variant_gen
}

#[proc_macro_error]
#[proc_macro_derive(EveryVariant)]
pub fn derive_every_variant(item: TokenStream) -> TokenStream {
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
                        let variant_gen = do_enum_gen(&varid, &fields.unnamed);
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
                    syn::Fields::Named(ref fields) => {
                        let variant_gen = do_enum_gen(&varid, &fields.named);
                        variant_generators.push(variant_gen);
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

            // println!("{}", out);
            out.into()
        }
        Item::Struct(ref it) => {
            // println!("struct: {:#?}", it);

            let name = &it.ident;

            //let mut member_generator = Vec::new();

            // This is used, but the compiler does not seem to detect it?
            #[allow(unused_assignments)]
            let mut structgen = quote! {};

            if it.fields.iter().any(|f| f.ident.is_none()) {
                let mut fieldgens = Vec::new();
                // Here we come if its an struct with anonymous fields
                for (idx, field) in it.fields.iter().enumerate() {
                    if let syn::Type::Path(path) = field.ty.clone() {
                        fieldgens.push(AnonStructFieldGen {
                            id: Ident::new(&format!("v{}", &idx.to_string()), Span::call_site()),
                            ty: path.clone(),
                        });
                    } else {
                        abort!(field, "Ident is missing");
                    }
                }

                let names: Vec<Ident> = fieldgens.iter().map(|f| f.id.clone()).collect();
                structgen = quote! {
                    let s = #name(
                      #( #names.clone() ),*
                    );
                    vec.push(s);
                };

                for field in fieldgens.iter().rev() {
                    let fname = &field.id;
                    let ftype = &field.ty;

                    structgen = quote! {
                        for #fname in <#ftype as EveryVariant>::every_variant() {
                            #structgen
                        }
                    };
                }
            } else {
                let mut fieldgens = Vec::new();
                for field in it.fields.iter() {
                    if let Some(name) = field.ident.clone() {
                        if let syn::Type::Path(path) = field.ty.clone() {
                            let fieldgen = StructFieldGen { name, ty: path };
                            fieldgens.push(fieldgen);
                        } else {
                            abort!(field, "Ident is missing");
                        }
                    }
                }

                let names: Vec<Ident> = fieldgens.iter().map(|f| f.name.clone()).collect();
                structgen = quote! {
                    let s = Self {
                      #( #names: #names.clone() ),*
                    };
                    vec.push(s);
                };

                for field in fieldgens.iter().rev() {
                    let fname = &field.name;
                    let ftype = &field.ty;
                    //println!("type: {}, dbg: {:?}", ftype.to_token_stream(), ftype);

                    structgen = quote! {
                        for #fname in <#ftype as EveryVariant>::every_variant() {
                            #structgen
                        }
                    };
                }
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

            // println!("{}", out);
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
