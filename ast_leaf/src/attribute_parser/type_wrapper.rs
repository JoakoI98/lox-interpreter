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

    pub fn validate_dual_tuple(&self, first_type: &str, second_type: Option<&str>) -> Type {
        let tuple_type = match &self.type_ {
            Type::Tuple(tuple) => tuple,
            _ => panic!("Dual tuple expected"),
        };
        if tuple_type.elems.len() != 2 {
            panic!("Dual tuple expected");
        };

        let first_type_wrapper = TypeWrapper::new(&tuple_type.elems[0]);
        let second_type_wrapper = TypeWrapper::new(&tuple_type.elems[1]);
        let first_type_check = first_type_wrapper.get_type_name() == Some(first_type.to_string());
        let second_type_check = second_type.is_none()
            || (second_type_wrapper.get_type_name() == Some(second_type.unwrap().to_string()));
        let result = first_type_check && second_type_check;
        if !result {
            panic!(
                "{} or {} type expected, got {:?} and {:?}",
                first_type,
                second_type.unwrap_or("None"),
                first_type_wrapper.get_type_name(),
                second_type_wrapper.get_type_name()
            );
        }
        tuple_type.elems[1].clone()
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
