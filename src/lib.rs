extern crate proc_macro;

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{self, parse_macro_input, parse_quote};

/// Add `#[serde(default)]` to the every field of struct
/// ## Examples
///
/// ```rust
/// use serde_default::serde_default;
/// use serde::{Deserialize, Serialize};
///
/// #[serde_default]
/// #[derive(Deserialize, Serialize)]
/// Struct Foo {
///   name: String,
///   age: int
/// }
/// ```
#[proc_macro_attribute]
pub fn serde_default(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut item: syn::DeriveInput = parse_macro_input!(input);
    let attr_add: syn::Attribute = parse_quote! {
        #[serde(default)]
    };
    let r = match &mut item.data {
        syn::Data::Struct(data) => {
            for f in data.fields.iter_mut() {
                let mut add_attr = true;
                for a in f.attrs.iter() {
                    if a.to_token_stream().to_string().contains("default")
                        && a.path().is_ident("serde")
                    {
                        add_attr = false;
                        break;
                    }
                }
                if add_attr {
                    f.attrs.push(attr_add.clone());
                }
            }
            item
        }
        _ => item,
    };
    r.to_token_stream().into()
}
