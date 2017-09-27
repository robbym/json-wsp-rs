extern crate proc_macro;
use proc_macro::TokenStream;

extern crate syn;

#[macro_use]
extern crate quote;

#[proc_macro_derive(WSPType)]
pub fn wsp_type_macro(input: TokenStream) -> TokenStream {
    let source = input.to_string();
    let ast = syn::parse_derive_input(&source).unwrap();
    let expanded_impl = expanded_impl(&ast);
    expanded_impl.parse().unwrap()
}

fn expanded_impl(ast: &syn::DeriveInput) -> quote::Tokens {
    let fields = match ast.body {
        syn::Body::Struct(syn::VariantData::Struct(ref fields)) => fields,
        _ => panic!("#[derive(WSPType)] can only be used with structs")
    };

    let name = &ast.ident;
    let field_names = fields.iter().map(|f| f.ident.clone());
    let field_types = fields.iter().map(|f| f.ty.clone());

    quote! {
        impl WSPType for #name {
            fn get_type() -> serde_json::Value {
                serde_json::Value::Object(
                    {
                        let mut map = serde_json::Map::new();
                        #(
                        map.insert(String::from(stringify!(#field_names)), <#field_types as WSPTypeMember>::get_type_member());
                        )*
                        map
                    }
                )
            }
        }
    }
}

#[proc_macro_derive(WSPTypeMember)]
pub fn wsp_type_member_macro(input: TokenStream) -> TokenStream {
    let source = input.to_string();
    let ast = syn::parse_derive_input(&source).unwrap();

    let name = &ast.ident;
    let expanded_impl = quote! {
        impl WSPTypeMember for #name {
            fn get_type_member() -> Value {
                Value::String(String::from(stringify!(#name)))
            }
        }
    };
    expanded_impl.parse().unwrap()
}

#[proc_macro_derive(WSPService)]
pub fn wsp_service_macro(input: TokenStream) -> TokenStream {
    let source = input.to_string();
    let ast = syn::parse_derive_input(&source).unwrap();

    let name = &ast.ident;
    let expanded_impl = quote! {
        impl WSPService for #name {
            fn get_service() -> Value {
                Value::String(String::new())
            }
        }
    };
    expanded_impl.parse().unwrap()
}