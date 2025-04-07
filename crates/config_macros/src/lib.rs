use proc_macro::TokenStream;
use quote::{ToTokens, quote, quote_spanned};
#[allow(unused_imports)]
use ron as johns_surf_shop;
use syn::{DeriveInput, LitStr, parse_macro_input};

#[proc_macro_derive(RonDefault, attributes(ron))]
pub fn default_ron(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident.clone();
    let dname = format!("{name}__RON_DERIVED_DEFAULT__");
    let default_val = syn::Ident::new(&dname, name.span());
    for attr in &input.attrs {
        if attr.path().is_ident("ron") {
            let path_arg: LitStr = attr.parse_args().unwrap();
            let path = path_arg.value();
            let root = std::env::var("CARGO_MANIFEST_DIR").unwrap();
            let path = std::path::Path::new(&root).join(path);
            if !path.exists() {
                let path = syn::LitStr::new(&path.to_string_lossy(), path_arg.span());
                return quote_spanned! {path.span() =>
                    compile_error!(concat!("unable to find file at ", #path));
                }
                .into();
            }
            let path = path.to_string_lossy();
            let path = syn::LitStr::new(&path, path_arg.span());
            return quote!(
                #[allow(non_upper_case_globals)]
                static #default_val: std::sync::LazyLock<#name> = std::sync::LazyLock::new(|| {
                    ron::de::from_reader(std::fs::File::open(#path).unwrap()).unwrap()
                });
                impl Default for #name {
                    fn default() -> Self {
                        // TODO: remove
                        let struct_name = #dname;
                        let start = std::time::Instant::now();
                        let value = #default_val.clone();
                        println!("clone for {} took {:?}", struct_name, start.elapsed());
                        value
                    }
                }
            )
            .into_token_stream()
            .into();
        }
    }
    quote_spanned!(input.ident.span() => compile_error!("ron_path was not found, specify a file path using #[ron(\"your/data/here.ron\")]");).into()
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
