extern crate proc_macro;
extern crate proc_macro2;

use proc_macro::TokenStream;
use quote::quote;
use syn::Data;

#[proc_macro_derive(IntoStringHashMap)]
pub fn derive_into_hash_map(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::DeriveInput);

    let struct_identifier = &input.ident;

    match &input.data {
        Data::Struct(syn::DataStruct { fields, .. }) => {
            // TokenStream is basically the inverse representation of an AST. You provide actual Rust code to the quote macro, and it gives us the "stream of tokens" as you've called it previously for that source code.
            let mut implementation = quote! {
                let mut hash_map = std::collections::HashMap::<String, String>::new();
            };

            // This TokenStream can either be converted to the macro's output type, or be manipulated using methods provided by quote such as extend.
            for field in fields {
                let identifier = field.ident.as_ref().unwrap();
                // # quote allow you to use any varaibles outside of the token stream with the #prefix
                implementation.extend(quote! {
                    hash_map.insert(
                        stringify!(#identifier).to_string(),
                        String::from(value.#identifier)
                    );
                });
            }

            /*
                let mut hash_map = std::collections::HashMap::<String, String>::new();
                hash_map.insert("username".to_string(), String::from(value.username));
                hash_map.insert("first_name".to_string(), String::from(value.first_name));
                hash_map.insert("last_name".to_string(), String::from(value.last_name));
            */

            quote! {
                #[automatically_derived]
                impl From<#struct_identifier> for std::collections::HashMap<String, String> {
                    fn from(value: #struct_identifier) -> Self {
                        #implementation

                        hash_map
                    }
                }
            }
        }
        _ => unimplemented!(),
    }
    .into()
}
