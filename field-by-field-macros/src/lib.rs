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
            let fn_fields_not_equal = build_fn_fields_not_equal(&data);
            let fn_assert_equal_field_by_field = build_fn_assert_equal_field_by_field(&name);

            quote! {
                impl #impl_generics ::field_by_field::EqualFieldByField for #name #ty_generics #where_clause {
                    #fn_fields_not_equal

                    #fn_assert_equal_field_by_field
                }
            }
        }
        syn::Body::Enum(_) => panic!("field_by_field can only be used with structs")
    }
}

/// Build a function that compares all the items in a simple struct
///
/// This emits just a long list of `if self.name != other.name {
/// vec.push(UnequalField); }` tokens.
fn build_fn_fields_not_equal(data: &syn::VariantData) -> quote::Tokens {
    let find_unequal_fields = data.fields().iter().map(|f| {
        let f_name = &f.ident;
        let f_str = f_name.as_ref().map(|v| v.to_string())
            .expect("Couldn't convert field to str");
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
