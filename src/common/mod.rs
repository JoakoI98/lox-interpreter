pub trait Visitor<T, U> {
    fn visit(&self, node: T) -> U;
}

pub trait Visitable {
    fn accept<'a, T, U: Visitor<&'a Self, T>>(&'a self, visitor: &U) -> T;
}
