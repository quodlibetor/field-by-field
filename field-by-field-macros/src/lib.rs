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
    expand_derive(input, field_by_field)
}

fn expand_derive(input: TokenStream, f: fn(syn::MacroInput) -> quote::Tokens) -> TokenStream {
    let item = syn::parse_macro_input(&input.to_string()).unwrap();
    f(item).to_string().parse().unwrap()
}

fn field_by_field(input: syn::MacroInput) -> quote::Tokens {
    let name = &input.ident;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    match input.body {
        syn::Body::Struct(ref data) => {
            let eqs = data.fields().iter().map(|f| {
                let f_name = &f.ident;
                let f_str = f_name.as_ref().map(|v| v.to_string())
                    .expect("Couldn't convert field to str");
                quote!{
                    if self.#f_name != other.#f_name {
                        list.push(format!("        {0}: {1:?} != {2:?}\n",
                                          #f_str, self.#f_name, other.#f_name));
                    }
                }
            });
            // For some reason breaking this out into its own quote section
            // prevents hitting the recursion depth limit
            let errmsgfmt = quote! {
                let ac_exp = format!("    actually: {:?}\n    \
                                      expected: {:?}\n", self, other);
                errmsg.push_str(&ac_exp);
            };
            quote! {
                impl #impl_generics ::field_by_field::EqualFieldByField for #name #ty_generics #where_clause {
                    fn assert_equal_field_by_field(&self, other: &#name) {
                        let mut list: Vec<String> = Vec::new();

                        #(#eqs)*

                        if list.len() > 0 {
                            let mut errmsg = String::from("\n    Items are not equal:\n");
                            for field_err in list {
                                errmsg.push_str(&field_err);
                            }

                            #errmsgfmt

                            panic!("{}", errmsg);
                        }
                    }
                }
            }
        }
        syn::Body::Enum(_) => panic!("field_by_field can only be used with structs")
    }
}
