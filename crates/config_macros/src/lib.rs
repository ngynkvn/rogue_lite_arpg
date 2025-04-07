use proc_macro::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::{Attribute, DeriveInput, Error, LitStr, parse_macro_input, spanned::Spanned};

#[proc_macro_derive(LazyRon, attributes(lazy, comptime))]
pub fn impl_lazy_ron(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    let mut output = TokenStream::new();
    let name = input.ident.clone();

    for attr in &input.attrs {
        if attr.path().is_ident("lazy") {
            let path = match get_lazy_path_attr(attr) {
                Ok(path) => path,
                Err(e) => return e.into_compile_error().into(),
            };
            output.extend(impl_static_default(&name, &path));
        }
    }

    if output.is_empty() {
        let attrs: proc_macro2::TokenStream = input
            .attrs
            .iter()
            .map(ToTokens::to_token_stream)
            .collect::<_>();
        let err = syn::Error::new_spanned(
            attrs,
            "ron path was not found.\nspecify a lazy path using #[lazy(\"your/data/here.ron\")]",
        )
        .into_compile_error();
        output.extend(TokenStream::from(err));
    }
    output
}

fn impl_static_default(name: &syn::Ident, path: &syn::LitStr) -> TokenStream {
    let derived_static = format_ident!("LAZY_DERIVED_{name}");
    let display_name = name.to_string();
    quote!(
        static #derived_static: std::sync::LazyLock<#name> = std::sync::LazyLock::new(|| {
            match ::ron::de::from_reader(std::fs::File::open(#path).unwrap()) {
                Ok(value) => value,
                Err(err) => panic!(
                    "encountered an error trying to parse {}\nerr: {}\npath: {}\n",
                    concat!(#display_name), err, #path)
            }
        });
        impl Default for #name {
            fn default() -> Self {
                #derived_static.clone()
            }
        }
    )
    .into_token_stream()
    .into()
}

fn get_lazy_path_attr(attr: &Attribute) -> Result<LitStr, Error> {
    let path_arg: LitStr = match attr.parse_args() {
        Ok(path_arg) => path_arg,
        Err(e) => {
            return Err(e);
        }
    };
    let path = path_arg.value();
    let root = std::env::var("CARGO_MANIFEST_DIR").map_err(|e| syn::Error::new(attr.span(), e))?;
    let path = std::path::Path::new(&root).join(path);
    if !path.exists() {
        return Err(Error::new(
            path_arg.span(),
            format!("unable to find ron file at {}", path.to_string_lossy()),
        ));
    }
    let path = path.to_string_lossy();
    let path = LitStr::new(&path, path_arg.span());
    Ok(path)
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
