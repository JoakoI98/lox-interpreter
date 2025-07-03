extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{Field, FieldMutability, Ident, Type, Visibility};
mod attribute_parser;
mod struct_parser;

fn push_struct_type_field(struct_ast: &mut syn::ItemStruct) {
    let field_name = Ident::new("Type", Span::call_site());
    let type_str = struct_ast.ident.to_string() + "_Type";
    let field_type = Ident::new(&type_str, Span::call_site());

    let field = Field {
        attrs: vec![],
        vis: Visibility::Inherited,
        ident: Some(field_name),
        ty: Type::Verbatim(quote! { #field_type }),
        colon_token: Some(Default::default()),
        mutability: FieldMutability::None,
    };

    match &mut struct_ast.fields {
        syn::Fields::Named(fields) => {
            fields.named.push(field);
        }
        _ => return,
    }
}

#[proc_macro_attribute]
pub fn ast_leaf(attr: TokenStream, item: TokenStream) -> TokenStream {
    let struct_ast = syn::parse_macro_input!(item as struct_parser::ASTLeafStruct);
    let mut attr_ast = syn::parse_macro_input!(attr as attribute_parser::Production);
    for (name, ty) in &struct_ast.non_terminal_fields {
        attr_ast = attr_ast.hydrate(name, ty.clone());
    }

    let enum_ast = attr_ast.get_enum(&struct_ast.type_field);

    let function_ast = attr_ast.get_parse_sentence(&struct_ast);

    let out = quote! {
        #enum_ast
        #struct_ast
        #function_ast
    };

    proc_macro::TokenStream::from(out)
}
