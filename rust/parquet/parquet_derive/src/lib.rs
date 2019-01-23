// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

#![recursion_limit = "200"]

extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use std::iter;
use syn::spanned::Spanned;
use syn::{
    parse_macro_input, AngleBracketedGenericArguments, Data, DataStruct, DeriveInput, Field,
    Fields, Ident, Lifetime, Meta, NestedMeta, Path, PathArguments, Type,
};

#[proc_macro_derive(Deserialize, attributes(parquet))]
pub fn parquet_deserialize(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let result = match ast.data {
        syn::Data::Struct(ref s) => new_for_struct(&ast, &s.fields),
        syn::Data::Enum(ref e) => new_for_enum(&ast, e),
        syn::Data::Union(_) => panic!("doesn't work with unions yet"),
    };
    let result = result.unwrap_or_else(|err| err.to_compile_error());
    result.into()
}

fn new_for_struct(ast: &syn::DeriveInput, fields: &syn::Fields) -> Result<TokenStream, syn::Error> {
    match *fields {
        syn::Fields::Named(ref fields) => new_impl(&ast, &fields.named, true),
        syn::Fields::Unit => new_impl(&ast, &syn::punctuated::Punctuated::new(), true),
        syn::Fields::Unnamed(ref fields) => new_impl(&ast, &fields.unnamed, false),
    }
}

fn new_for_enum(ast: &syn::DeriveInput, data: &syn::DataEnum) -> Result<TokenStream, syn::Error> {
    if data.variants.is_empty() {
        panic!("#[derive(Deserialize)] cannot be implemented for enums with zero variants");
    }
    unimplemented!()
    // let impls = data.variants.iter().map(|v| {
    //     if v.discriminant.is_some() {
    //         panic!("#[derive(new)] cannot be implemented for enums with discriminants");
    //     }
    //     new_for_struct(ast, &v.fields, Some(&v.ident))
    // });
    // my_quote!(#(#impls)*)
}

fn new_impl(
    ast: &syn::DeriveInput,
    fields: &syn::punctuated::Punctuated<syn::Field, Token![,]>,
    named: bool,
) -> Result<TokenStream, syn::Error> {
    let name = &ast.ident;
    let schema_name = Ident::new(&format!("{}Schema", name), Span::call_site());
    let reader_name = Ident::new(&format!("{}Reader", name), Span::call_site());

    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let field_renames1 = fields
        .iter()
        .map(|field| {
            let mut rename = None;
            for meta_items in field.attrs.iter().filter_map(get_parquet_meta_items) {
                for meta_item in meta_items {
                    match meta_item {
                        // Parse `#[parquet(rename = "foo")]`
                        NestedMeta::Meta(Meta::NameValue(ref m)) if m.ident == "rename" => {
                            let s = get_lit_str(&m.ident, &m.ident, &m.lit)?;
                            if rename.is_some() {
                                return Err(syn::Error::new_spanned(
                                    &m.ident,
                                    "duplicate parquet attribute `rename`",
                                ));
                            }
                            rename = Some(s.clone());
                        }
                        NestedMeta::Meta(ref meta_item) => {
                            return Err(syn::Error::new_spanned(
                                meta_item.name(),
                                format!("unknown parquet field attribute `{}`", meta_item.name()),
                            ));
                        }
                        NestedMeta::Literal(ref lit) => {
                            return Err(syn::Error::new_spanned(
                                lit,
                                "unexpected literal in parquet field attribute",
                            ));
                        }
                    }
                }
            }
            Ok(rename.unwrap_or_else(|| {
                syn::LitStr::new(&field.ident.as_ref().unwrap().to_string(), field.span())
            }))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let field_renames2 = field_renames1.clone();
    let field_renames3 = field_renames1.clone();

    let field_names1 = fields.iter().map(|field| field.ident.as_ref().unwrap());
    let field_names2 = fields.iter().map(|field| field.ident.as_ref().unwrap());
    let field_names3 = fields.iter().map(|field| field.ident.as_ref().unwrap());
    let field_names4 = fields.iter().map(|field| field.ident.as_ref().unwrap());
    let field_names5 = fields.iter().map(|field| field.ident.as_ref().unwrap());
    let field_names6 = fields.iter().map(|field| field.ident.as_ref().unwrap());
    let field_names7 = fields.iter().map(|field| field.ident.as_ref().unwrap());
    let field_names8 = fields.iter().map(|field| field.ident.as_ref().unwrap());
    let field_names9 = fields.iter().map(|field| field.ident.as_ref().unwrap());
    let field_names10 = fields.iter().map(|field| field.ident.as_ref().unwrap());
    let field_names11 = fields.iter().map(|field| field.ident.as_ref().unwrap());
    let field_names12 = fields.iter().map(|field| field.ident.as_ref().unwrap());

    let field_types1 = fields.iter().map(|field| &field.ty);
    let field_types2 = fields.iter().map(|field| &field.ty);
    let field_types3 = fields.iter().map(|field| &field.ty);
    let field_types4 = fields.iter().map(|field| &field.ty);

    let name1 = iter::repeat(name);

    let gen = quote! {
        use _parquet::{
            basic::Repetition,
            column::reader::ColumnReader,
            errors::ParquetError,
            record::reader::Reader,
            record::Deserialize,
            record::DisplaySchema,
            schema::types::ColumnDescPtr,
            schema::types::{ColumnPath, Type},
        };
        use ::std::{collections::HashMap, fmt, result::Result, string::String, vec::Vec};

        #[derive(Debug)]
        struct #schema_name {
            #(#field_names1: <#field_types1 as Deserialize>::Schema,)*
        }
        impl DisplaySchema for #schema_name {
            fn fmt(&self, r: Repetition, name: &str, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
                f.debug_struct(stringify!(#schema_name))
                    // $(.field(stringify!($name), &DisplayDisplayType::<<$type_ as Deserialize>::Schema>::new()))*
                    .finish()
            }
            fn fmt_type(r: Repetition, name: &str, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
                f.debug_struct(stringify!(#schema_name))
                    // $(.field(stringify!($name), &DisplayDisplayType::<<$type_ as Deserialize>::Schema>::new()))*
                    .finish()
            }
        }
        struct #reader_name {
            #(#field_names2: <#field_types2 as Deserialize>::Reader,)*
        }
        impl Reader for #reader_name {
            type Item = #name;

            fn read(&mut self) -> Result<Self::Item, ParquetError> {
                Result::Ok(#name {
                    #(#field_names3: self.#field_names4.read()?,)*
                })
            }
            fn advance_columns(&mut self) -> Result<(), ParquetError> {
                #(self.#field_names5.advance_columns()?;)*
                Result::Ok(())
            }
            fn has_next(&self) -> bool {
                #(if true { self.#field_names6.has_next() } else)*
                {
                    true
                }
            }
            fn current_def_level(&self) -> i16 {
                #(if true { self.#field_names7.current_def_level() } else)*
                {
                    panic!("Current definition level: empty group reader")
                }
            }
            fn current_rep_level(&self) -> i16 {
                #(if true { self.#field_names8.current_rep_level() } else)*
                {
                    panic!("Current repetition level: empty group reader")
                }
            }
        }
        impl #impl_generics Deserialize for #name #ty_generics #where_clause {
            type Schema = #schema_name;
            type Reader = #reader_name;

            fn parse(schema: &Type) -> Result<(String,Self::Schema),ParquetError> {
                if schema.is_group() && !schema.is_schema() && schema.get_basic_info().repetition() == Repetition::REQUIRED {
                    let fields = schema.get_fields().iter().map(|field|(field.name(),field)).collect::<HashMap<_,_>>();
                    let schema_ = #schema_name{
                        #(#field_names9: fields.get(#field_renames1).ok_or(ParquetError::General(format!("Struct {} missing field {}", stringify!(#name1), #field_renames2))).and_then(|x|<#field_types3 as Deserialize>::parse(&**x))?.1,)*
                    };
                    return Result::Ok((schema.name().to_owned(), schema_))
                }
                Result::Err(ParquetError::General(format!("Struct {}", stringify!(#name))))
            }
            fn reader(schema: &Self::Schema, mut path: &mut Vec<String>, curr_def_level: i16, curr_rep_level: i16, paths: &mut HashMap<ColumnPath, (ColumnDescPtr,ColumnReader)>, batch_size: usize) -> Self::Reader {
                #(
                    path.push(#field_renames3.to_owned());
                    let #field_names10 = <#field_types4 as Deserialize>::reader(&schema.#field_names11, path, curr_def_level, curr_rep_level, paths, batch_size);
                    path.pop().unwrap();
                )*
                #reader_name { #(#field_names12,)* }
            }
        }
    };

    Ok(wrap_in_const("DESERIALIZE", name, gen))
}

fn get_parquet_meta_items(attr: &syn::Attribute) -> Option<Vec<syn::NestedMeta>> {
    if attr.path.segments.len() == 1 && attr.path.segments[0].ident == "parquet" {
        match attr.interpret_meta() {
            Some(Meta::List(ref meta)) => Some(meta.nested.iter().cloned().collect()),
            _ => {
                // TODO: produce an error
                None
            }
        }
    } else {
        None
    }
}

fn get_lit_str<'a>(
    attr_name: &Ident,
    meta_item_name: &Ident,
    lit: &'a syn::Lit,
) -> Result<&'a syn::LitStr, syn::Error> {
    if let syn::Lit::Str(ref lit) = *lit {
        Ok(lit)
    } else {
        Err(syn::Error::new_spanned(
            lit,
            format!(
                "expected parquet {} attribute to be a string: `{} = \"...\"`",
                attr_name, meta_item_name
            ),
        ))
    }
}

fn wrap_in_const(trait_: &str, ty: &Ident, code: TokenStream) -> TokenStream {
    let dummy_const = Ident::new(
        &format!("_IMPL_{}_FOR_{}", trait_, unraw(ty)),
        Span::call_site(),
    );

    quote! {
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const #dummy_const: () = {
            #[allow(unknown_lints)]
            #[cfg_attr(feature = "cargo-clippy", allow(useless_attribute))]
            #[allow(rust_2018_idioms)]
            extern crate parquet as _parquet;
            #code
        };
    }
}

fn unraw(ident: &Ident) -> String {
    ident.to_string().trim_start_matches("r#").to_owned()
}