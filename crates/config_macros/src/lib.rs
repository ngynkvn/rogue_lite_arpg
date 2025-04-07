use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    DeriveInput, Expr, ExprLit, Lit, LitStr, MetaNameValue, parse_macro_input,
    punctuated::Punctuated,
};

#[proc_macro_derive(RonDefault, attributes(ron))]
pub fn default_ron(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let dname = format!("_DEFAULT{name}");
    let default_val = syn::Ident::new(&dname, name.span());
    for attr in &input.attrs {
        if attr.path().is_ident("ron") {
            let path: LitStr = attr.parse_args().unwrap();
            let path = path.value();
            let root = env!("CARGO_MANIFEST_DIR");
            let path = std::path::Path::new(root).join(path);
            let data = std::fs::read_to_string(path).unwrap();
            return quote!(
                #[allow(non_upper_case_globals)]
                static #default_val: std::sync::LazyLock<#name> = std::sync::LazyLock::new(|| {
                    ron::de::from_str(#data).unwrap()
                });
                #[allow(non_upper_case_globals)]
                impl Default for #name {
                    fn default() -> Self {
                        *#default_val
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
