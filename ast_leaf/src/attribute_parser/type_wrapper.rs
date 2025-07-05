use std::fmt::Debug;
use syn::Type;

pub struct TypeWrapper<'a> {
    type_: &'a Type,
}

impl<'a> TypeWrapper<'a> {
    pub fn new(type_: &'a Type) -> Self {
        Self { type_ }
    }

    pub fn get_type_name(&self) -> Option<String> {
        match &self.type_ {
            Type::Path(path) => Some(path.path.segments.last().unwrap().ident.to_string()),
            _ => None,
        }
    }

    pub fn validate_type(&self, expected_type: &str, inner_type_index: usize) -> Type {
        if self.get_type_name() != Some(expected_type.to_string()) {
            panic!(
                "{} type expected, got {:?}",
                expected_type,
                self.get_type_name()
            );
        }
        let inner_type = self.get_inner_type();
        if inner_type.is_none() {
            panic!("Inner type expected");
        }
        let inner_type = inner_type.unwrap();
        if inner_type.len() <= inner_type_index {
            panic!(
                "Inner type expected to be at least {} types",
                inner_type_index
            );
        }
        inner_type[inner_type_index].clone()
    }

    pub fn get_inner_type(&self) -> Option<Vec<Type>> {
        match &self.type_ {
            Type::Path(path) => {
                let segment = path.path.segments.last()?;
                match &segment.arguments {
                    syn::PathArguments::AngleBracketed(angle_bracketed) => Some(
                        angle_bracketed
                            .args
                            .iter()
                            .filter_map(|f| match f {
                                syn::GenericArgument::Type(ty) => Some(ty.clone()),
                                _ => None,
                            })
                            .collect(),
                    ),
                    _ => None,
                }
            }
            _ => None,
        }
    }
}

impl<'a> Debug for TypeWrapper<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.type_ {
            Type::Path(path) => {
                write!(f, "Path type: ")?;
                let segments = &path.path.segments;
                for segment in segments {
                    match &segment.arguments {
                        syn::PathArguments::None => {}
                        syn::PathArguments::AngleBracketed(angle_bracketed) => {
                            write!(f, " Braced")?;
                            for arg in &angle_bracketed.args {
                                match arg {
                                    syn::GenericArgument::Type(ty) => {
                                        write!(f, " {:?}", TypeWrapper::new(&ty))?;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        syn::PathArguments::Parenthesized(_) => {
                            write!(f, " Parenthesized")?;
                        }
                    }
                    write!(f, " {}", segment.ident.to_string())?;
                    write!(f, "\n")?;
                }
            }
            _ => {
                write!(f, "Type")?;
            }
        }
        Ok(())
    }
}
