extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
mod attribute_parser;
mod struct_parser;

#[proc_macro_attribute]
pub fn ast_leaf(attr: TokenStream, item: TokenStream) -> TokenStream {
    let struct_ast = syn::parse_macro_input!(item as struct_parser::ASTLeafStruct);
    let mut attr_ast = syn::parse_macro_input!(attr as attribute_parser::Production);
    let dummy_type = syn::Type::Verbatim(quote! {String});
    attr_ast = attr_ast.hydrate("dummy", dummy_type, &struct_ast.type_field);

    for (name, ty) in &struct_ast.non_terminal_fields {
        attr_ast = attr_ast.hydrate(name, ty.clone(), &struct_ast.type_field);
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
