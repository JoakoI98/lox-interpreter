extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as ProcMacro2TokenStream};
use quote::quote;
use syn::{Field, FieldMutability, Ident, Type, Visibility};
mod attribute_parser;

fn push_struct_type_field(struct_ast: &syn::ItemStruct) -> ProcMacro2TokenStream {
    let field_name = Ident::new("Type", Span::call_site());
    let type_str = struct_ast.ident.to_string() + "_Type";
    let field_type = Ident::new(&type_str, Span::call_site());
    let other_fields = match &struct_ast.fields {
        syn::Fields::Named(fields) => {
            let named = &fields.named;
            quote! {#named}
        }
        _ => {
            quote! {}
        }
    };
    let struct_ident = struct_ast.ident.clone();

    let field = quote! {
        #field_name: #field_type,
    };

    return quote! {
        struct #struct_ident {
            #other_fields
            #field
        }
    };
}

#[proc_macro_attribute]
pub fn ast_leaf(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut struct_ast = syn::parse_macro_input!(item as syn::ItemStruct);
    let struct_tokens = push_struct_type_field(&struct_ast);
    let attr_ast = syn::parse_macro_input!(attr as attribute_parser::Production);

    let enum_ast = attr_ast.get_enum(&struct_ast.ident.to_string());

    let out = quote! {
        #enum_ast
        #struct_tokens
    };

    proc_macro::TokenStream::from(out)
}
