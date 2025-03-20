use std::any::Any;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Data, Fields, Lit, Meta, PatLit, Token, parse::ParseStream, punctuated::Punctuated,
    spanned::Spanned,
};

#[proc_macro_derive(AssetGroup, attributes(asset))]
pub fn asset_group_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_asset_group(&ast)
}

const PATH_ASSET: &str = "asset";
const PATH_SRC: &str = "src";

// Structure to hold asset field data
struct AssetField {
    field_name: syn::Ident,
    field_type: syn::Type,
    src_path: String,
}

fn impl_asset_group(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let Data::Struct(ref data_struct) = ast.data else {
        panic!("bad");
    };
    let Fields::Named(ref named_fields) = data_struct.fields else {
        panic!("bad2");
    };
    let mut assets: Vec<AssetField> = vec![];
    for field in named_fields.named.iter() {
        match parse_asset_attr(field) {
            Ok(asset_field) => {
                assets.push(asset_field);
            }
            Err(err) => {
                return err.to_compile_error().into();
            }
        }
    }

    // Generate field assignments for the struct initialization
    let field_assignments = assets.iter().map(|asset| {
        let field_name = &asset.field_name;
        let src_path = &asset.src_path;
        quote! {
            #field_name: server.load(#src_path)
        }
    });

    let r#gen = quote! {
        impl AssetGroup for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }

            fn load_assets_system(mut commands: ::bevy::ecs::system::Commands, server: ::bevy::ecs::system::Res<::bevy::asset::AssetServer>) {
                let #name = #name {
                    #(#field_assignments,)*
                };
                commands.insert_resource(#name);
            }
        }
    };
    r#gen.into()
}

fn parse_asset_attr(field: &syn::Field) -> Result<AssetField, syn::Error> {
    let field_name = field.ident.clone().expect("Field must have a name");
    let field_type = field.ty.clone();
    let mut src_path = None;

    let attrs = field
        .attrs
        .iter()
        .filter(|attr| attr.path().is_ident(PATH_ASSET));

    for attr in attrs {
        let asset_properties =
            attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;
        for prop in asset_properties.iter() {
            let prop = prop.require_name_value()?;
            if !prop.path.is_ident(PATH_SRC) {
                return Err(syn::Error::new(
                    prop.path.span(),
                    format!(
                        "expected `{PATH_SRC} but got {:?}`",
                        prop.path.get_ident().unwrap()
                    ),
                ));
            }
            let syn::Expr::Lit(syn::ExprLit {
                lit: Lit::Str(ref lit),
                ..
            }) = prop.value
            else {
                return Err(syn::Error::new(
                    prop.value.span(),
                    format!("a string literal but got {:?}`", prop.value.type_id()),
                ));
            };

            src_path = Some(lit.value());
        }
    }

    if let Some(src_path) = src_path {
        Ok(AssetField {
            field_name,
            field_type,
            src_path,
        })
    } else {
        Err(syn::Error::new(
            field.span(),
            "Field with #[asset] attribute must have src property",
        ))
    }
}
