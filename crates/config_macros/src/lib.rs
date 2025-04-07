use proc_macro::TokenStream;
use quote::{ToTokens, quote, quote_spanned};
use ron;
use syn::{DeriveInput, LitStr, parse_macro_input};

#[proc_macro_derive(RonDefault, attributes(ron))]
pub fn default_ron(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let dname = format!("{name}__RON_DERIVED_DEFAULT__");
    let default_val = syn::Ident::new(&dname, name.span());
    for attr in &input.attrs {
        if attr.path().is_ident("ron") {
            let path_arg: LitStr = attr.parse_args().unwrap();
            let path = path_arg.value();
            let root = std::env::var("CARGO_MANIFEST_DIR").unwrap();
            let path = std::path::Path::new(&root).join(path);
            if !path.exists() {
                let path = path.to_string_lossy();
                return quote_spanned! {path_arg.span() => compile_error!(format!("{} was not found.", #path))}.into();
            }
            let path = path.to_string_lossy();
            let path = syn::LitStr::new(&path, path_arg.span());
            return quote!(
                #[allow(non_upper_case_globals)]
                static #default_val: std::sync::LazyLock<#name> = std::sync::LazyLock::new(|| {
                    ron::de::from_reader(std::fs::File::open(#path).unwrap()).unwrap()
                });
                #[allow(non_upper_case_globals)]
                impl Default for #name {
                    fn default() -> Self {
                        #default_val.clone()
                    }
                }
            )
            .into_token_stream()
            .into();
        }
    }
    let input = quote!(compile_error!("ron_path was not found"););
    input.into_token_stream().into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_runner() {
        let t = trybuild::TestCases::new();
        t.pass("tests_macro/pass/*.rs");
        t.compile_fail("tests_macro/*.rs");
    }
}
