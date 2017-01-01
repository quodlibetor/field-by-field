#![feature(proc_macro, proc_macro_lib)]

/// Custom-derive providing macros for the field-by-field Traits
///
/// See the tests/derive.rs file for examples of use.

extern crate field_by_field;
extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;


#[proc_macro_derive(FieldByField)]
pub fn derive(input: TokenStream) -> TokenStream {
    expand_derive(input, build_trait_field_by_field)
}

fn expand_derive(input: TokenStream, f: fn(syn::MacroInput) -> quote::Tokens) -> TokenStream {
    let item = syn::parse_macro_input(&input.to_string()).unwrap();
    f(item).to_string().parse().unwrap()
}

fn build_trait_field_by_field(input: syn::MacroInput) -> quote::Tokens {
    let name = &input.ident;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    match input.body {
        syn::Body::Struct(ref data) => {
            let fn_fields_not_equal = build_fn_fields_not_equal(&data.fields());
            let fn_assert_equal_field_by_field = build_fn_assert_equal_field_by_field(&name);

            quote! {
                impl #impl_generics ::field_by_field::EqualFieldByField
                for #name #ty_generics #where_clause {
                    #fn_fields_not_equal

                    #fn_assert_equal_field_by_field
                }
            }
        }
        syn::Body::Enum(ref data) => {
            let fn_assert = build_fn_assert_equal_field_by_field(&name);
            let is_multivariant = data.len() > 1;
            let variants = data.iter()
                .map(|var| {
                    let var_name = &var.ident;
                    match var.data {
                        syn::VariantData::Unit =>
                            build_match_unit_variant(&name, &var_name, is_multivariant),
                        syn::VariantData::Tuple(ref fields) =>
                            build_match_tuple_variant(&name, &var_name, fields, is_multivariant),
                        syn::VariantData::Struct(_) =>
                            panic!("Didn't make it to struct enums yet"),
                    }
                }
            );

            quote! {
                impl #impl_generics ::field_by_field::EqualFieldByField
                for #name #ty_generics #where_clause {

                    fn fields_not_equal(&self, other: &Self)
                    -> Vec<::field_by_field::UnequalField> {
                        // This is never modified in the single-variant enum case.
                        #[allow(unused_mut)]
                        let mut list: Vec<::field_by_field::UnequalField> = Vec::new();

                        match (self, other) {
                            #(#variants)*
                        }

                        list
                    }

                    #fn_assert
                }
            }
        }
    }
}


/// Build a function that compares all the items in a simple struct
///
/// This emits just a long list of `if self.name != other.name {
/// vec.push(UnequalField); }` tokens.
fn build_fn_fields_not_equal(fields: &[syn::Field]) -> quote::Tokens {
    let find_unequal_fields = fields.iter().map(|f| {
        let f_name = &f.ident;
        let f_str = f_name.as_ref().map(|v| v.to_string())
            .expect(&format!("Couldn't convert field to str: {:?}", f));
        quote! {
            if self.#f_name != other.#f_name {
                list.push(::field_by_field::UnequalField {
                    field_name: #f_str.to_string(),
                    actually: Box::new(self.#f_name.clone()),
                    expected: Box::new(other.#f_name.clone()),
                });
            }
        }
    });

    quote! {
        fn fields_not_equal(&self, other: &Self)
                            -> Vec<::field_by_field::UnequalField>
        {
            let mut list: Vec<::field_by_field::UnequalField> = Vec::new();

            #(#find_unequal_fields)*

            list
        }
    }
}

/// Build a match statement that compares the self variant to the other variant
///
/// Since this is for unit variants this doesn't check the actual value if it
/// isn't an exact match.
fn build_match_unit_variant(name: &syn::Ident, var_name: &syn::Ident, is_multivariant: bool)
-> quote::Tokens {
    let left_str = format!("{}::{}", &name, &var_name);

    if is_multivariant {
        quote! {
            ( &#name::#var_name, &#name::#var_name ) => {}
            ( &#name::#var_name, ref expected ) => {
                list.push(::field_by_field::UnequalField {
                    field_name: #left_str.to_string(),
                    actually: Box::new(#left_str.to_string()),
                    expected: Box::new(format!("{:?}", &expected)),
                });
            }
        }
    } else {
        // TODO: This causes a "variable does not need to be mutable" warning
        quote! {
            ( &#name::#var_name, &#name::#var_name ) => {}
        }
    }
}

/// Build a match statement that compares a tuple enum against others
fn build_match_tuple_variant(name: &syn::Ident,
                             var_name: &syn::Ident,
                             var_fields: &[syn::Field],
                             is_multivariant: bool)
-> quote::Tokens {
    let actually_fields: Vec<_> = var_fields.iter()
        .enumerate()
        .map(|(i, f)| {
            assert!(f.ident.is_none(),
                    format!("Fields in tuples should be unnamed, not {:?}", f.ident));
            syn::Ident::from(format!("_{}", i))
        })
        .collect();
    let actually_field_refs: Vec<_> = actually_fields.iter()
        .map(|name| quote! { ref #name }).collect();
    let expected_fields: Vec<_> = var_fields.iter()
        .enumerate()
        .map(|(i, f)| {
            assert!(f.ident.is_none(),
                    format!("Fields in tuples should be unnamed, not {:?}", f.ident));
            syn::Ident::from(format!("_e{}", i))
        })
        .collect();
    let expected_field_refs: Vec<_> = expected_fields.iter()
        .map(|name| quote! { ref #name }).collect();

    let name_str = name.to_string();
    let var_name_str = var_name.to_string();

    let comparisons: Vec<_> = actually_fields.iter()
        .zip(&expected_fields)
        .enumerate()
        .map(|(i, (actually, expected))| {
             let field_name = format!("{}::{}.{}", name_str, var_name_str, i);
             quote! {
                 if #actually != #expected {
                     list.push(::field_by_field::UnequalField {
                         field_name: #field_name.into(),
                         actually: Box::new(#actually.clone()),
                         expected: Box::new(#expected.clone()),
                     });
                 }
             }
        })
        .collect();

    let field_match = quote! {
        ( &#name::#var_name(#(#actually_field_refs),*),
          &#name::#var_name(#(#expected_field_refs),*) ) => {
            #(#comparisons)*
        }
    };

    if is_multivariant {
        let var_field_name = format!("{}::{}", name_str, var_name_str);
        // TODO: improve error messages if the variants differ
        quote! {
            #field_match
            (ref actually @ &#name::#var_name(..), ref expected) => {
                list.push(::field_by_field::UnequalField {
                    field_name: #var_field_name.into(),
                    actually: Box::new((*actually).clone()),
                    expected: Box::new((*expected).clone()),
                });
            }
        }
    } else {
        field_match
    }
}


/// Build a function that panics if the result of fiels_not_equal is non-empty
///
/// The Function will panic with a nice error message
fn build_fn_assert_equal_field_by_field(name: &syn::Ident) -> quote::Tokens {
    let overall_errmsgfmt = quote! {
        let ac_exp = format!("    actually: {:?}\n    \
                              expected: {:?}\n", self, other);
        errmsg.push_str(&ac_exp);
    };

    let field_errmsgfmt = quote! {
        errmsg.push_str(&format!("        {}: {:?} != {:?}\n",
                                 field_err.field_name,
                                 field_err.actually, field_err.expected));

    };

    quote! {
        fn assert_equal_field_by_field(&self, other: &#name) {
            let errs = self.fields_not_equal(other);

            if errs.len() > 0 {
                let mut errmsg = String::from("\n    Items are not equal:\n");
                for field_err in errs {
                    #field_errmsgfmt
                }

                #overall_errmsgfmt

                panic!("{}", errmsg);
            }
        }
    }
}
