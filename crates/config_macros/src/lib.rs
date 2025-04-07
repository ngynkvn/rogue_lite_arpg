use proc_macro::TokenStream;
use quote::{ToTokens, quote, quote_spanned};
#[allow(unused_imports)]
use ron as johns_surf_shop;
use syn::{DeriveInput, LitStr, parse_macro_input};

#[proc_macro_derive(DefaultRon, attributes(ron))]
pub fn impl_ron_default_ron(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident.clone();
    let dname = format!("{name}__RON_DERIVED_DEFAULT__");
    let default_val = syn::Ident::new(&dname, name.span());
    for attr in &input.attrs {
        if attr.path().is_ident("ron") {
            let path_arg: LitStr = match attr.parse_args() {
                Ok(path_arg) => path_arg,
                Err(e) => {
                    return e.to_compile_error().into();
                }
            };
            let path = path_arg.value();
            let root = std::env::var("CARGO_MANIFEST_DIR").unwrap();
            let path = std::path::Path::new(&root).join(path);
            if !path.exists() {
                return syn::Error::new(
                    path_arg.span(),
                    format!("unable to find file at {}", path.to_string_lossy()),
                )
                .into_compile_error()
                .into();
            }
            let path = path.to_string_lossy();
            let path = syn::LitStr::new(&path, path_arg.span());
            return quote!(
                #[allow(non_upper_case_globals)]
                static #default_val: std::sync::LazyLock<#name> = std::sync::LazyLock::new(|| {
                    match ron::de::from_reader(std::fs::File::open(#path).unwrap()) {
                        Ok(value) => value,
                        Err(err) => panic!(
                            "encountered an error trying to parse {}\npath: {}\nerr: {}",
                            std::any::type_name::<#name>(), #path, err)
                    }
                });
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
