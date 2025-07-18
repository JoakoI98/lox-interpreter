pub trait Visitor<T, U> {
    fn visit(&self, node: T) -> U;
}

pub trait VisitorWithContext<T, U, C> {
    fn visit_with_context(&self, node: T, context: &C) -> U;
}

pub trait Visitable {
    fn accept<'a, T, U: Visitor<&'a Self, T>>(&'a self, visitor: &U) -> T {
        visitor.visit(self)
    }
    fn accept_with_context<'a, T, U: VisitorWithContext<&'a Self, T, C>, C>(
        &'a self,
        visitor: &U,
        context: &C,
    ) -> T {
        visitor.visit_with_context(self, context)
    }
}
