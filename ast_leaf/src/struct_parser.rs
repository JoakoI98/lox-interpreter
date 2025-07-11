use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    spanned::Spanned,
    Ident, ItemStruct, Result, Type,
};

pub struct ASTLeafStruct {
    pub name: String,
    pub type_field: String,
    pub type_field_ident: Ident,
    struct_ast: ItemStruct,
    pub non_terminal_fields: Vec<(String, Type)>,
    pub token_list_field: Option<Ident>,
    pub remap_error: Option<syn::LitStr>,
}

const TYPE_FIELD_ATTR: &str = "Type";
const TOKEN_LIST_FIELD_ATTR: &str = "TokenList";
const REMAPPED_ERROR_ATTR: &str = "SyncError";

impl Parse for ASTLeafStruct {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut struct_ast = input.parse::<ItemStruct>()?;
        let name = struct_ast.ident.to_string();
        let type_field = struct_ast.fields.iter_mut().find_map(|f| {
            let non_type_attrs = f.attrs.iter().filter(|&attr| {
                !(attr.meta.path().segments.iter().any(|f| {
                    return f.ident.to_string() == TYPE_FIELD_ATTR;
                }))
            });
            let non_type_attrs_cloned = non_type_attrs.cloned().collect::<Vec<_>>();
            if non_type_attrs_cloned.len() < f.attrs.len() {
                f.attrs = non_type_attrs_cloned;
                return Some(f);
            }
            return None;
        });
        if type_field.is_none() {
            return Err(syn::Error::new(
                struct_ast.ident.span(),
                "Type field not found annotate it with #[Type]",
            ));
        }
        let type_field = type_field.unwrap();
        let type_field_name = match &type_field.ty {
            Type::Path(path) => {
                let path_segments = path
                    .path
                    .segments
                    .iter()
                    .map(|s| s.ident.to_string())
                    .collect::<Vec<_>>();
                if path_segments.len() != 1 {
                    return Err(syn::Error::new(
                        path.span(),
                        "Type field must be a single path segment",
                    ));
                }
                path_segments[0].clone()
            }
            _ => {
                return Err(syn::Error::new(
                    type_field.span(),
                    "Type field must be a path",
                ));
            }
        };

        let type_field_ident_string = type_field
            .ident
            .as_ref()
            .map(|ident| ident.to_string())
            .unwrap_or("".to_string());

        let type_field_ident = Ident::new(&type_field_ident_string, type_field.span());

        let t = struct_ast
            .fields
            .iter()
            .filter_map(|f| {
                let f_name = f
                    .ident
                    .as_ref()
                    .map(|i| i.to_string())
                    .unwrap_or("".to_string());
                if f_name == type_field_ident_string {
                    return None;
                }
                let f_ty = f.ty.clone();
                return Some((f_name, f_ty));
            })
            .collect::<Vec<_>>();

        let token_field = struct_ast.fields.iter_mut().find_map(|f| {
            let non_type_attrs = f.attrs.iter().filter(|&attr| {
                !(attr.meta.path().segments.iter().any(|f| {
                    return f.ident.to_string() == TOKEN_LIST_FIELD_ATTR;
                }))
            });
            let non_type_attrs_cloned = non_type_attrs.cloned().collect::<Vec<_>>();
            if non_type_attrs_cloned.len() < f.attrs.len() {
                f.attrs = non_type_attrs_cloned;
                return Some(f);
            }
            return None;
        });

        let token_list_field = token_field.map(|f| f.ident.clone()).flatten();

        let mut remap_error = None;

        let remap_attrs: Vec<_> = struct_ast
            .attrs
            .into_iter()
            .filter(|attr| {
                let meta_attr = attr.meta.require_name_value();
                if let Ok(meta_attr) = meta_attr {
                    let is_remap_error = meta_attr
                        .path
                        .segments
                        .iter()
                        .any(|s| s.ident.to_string() == REMAPPED_ERROR_ATTR);
                    if !is_remap_error {
                        return true;
                    }
                    match &meta_attr.value {
                        syn::Expr::Lit(expr_lit) => match &expr_lit.lit {
                            syn::Lit::Str(lit_str) => {
                                remap_error = Some(lit_str.clone());
                                return false;
                            }
                            _ => panic!("Remapped error attribute must be a string"),
                        },
                        _ => panic!("Remapped error attribute must be a string"),
                    }
                }
                return true;
            })
            .collect();
        struct_ast.attrs = remap_attrs;

        Ok(ASTLeafStruct {
            name,
            type_field: type_field_name,
            type_field_ident,
            struct_ast,
            non_terminal_fields: t,
            token_list_field,
            remap_error,
        })
    }
}

impl ToTokens for ASTLeafStruct {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let struct_ast = &self.struct_ast;
        tokens.extend(quote! {
            #struct_ast
        });
    }
}
