use super::declaration_builders::RunnableBuilder;
use crate::common::{Visitable, VisitorWithContext};
use crate::evaluation::run::runnable::{BlockRunnable, ProgramRunnable, Runnable};
use crate::evaluation::runtime_value::Result;
use crate::evaluation::BuilderContext;
use crate::syntax_analysis::{Block, ProgramAst};

impl VisitorWithContext<&ProgramAst, Result<Box<dyn Runnable>>, BuilderContext>
    for RunnableBuilder
{
    fn visit_with_context(
        &self,
        node: &ProgramAst,
        context: &BuilderContext,
    ) -> Result<Box<dyn Runnable>> {
        let statements = node
            .statements
            .iter()
            .map(|(_, stmt)| stmt.accept_with_context(&Self, context))
            .collect::<Result<Vec<Box<dyn Runnable>>>>()?;
        Ok(Box::new(ProgramRunnable::new(statements)))
    }
}

impl VisitorWithContext<&Block, Result<Box<dyn Runnable>>, BuilderContext> for RunnableBuilder {
    fn visit_with_context(
        &self,
        node: &Block,
        context: &BuilderContext,
    ) -> Result<Box<dyn Runnable>> {
        context.resolver.borrow_mut().enter_scope()?;
        let declarations = node
            .declarations
            .iter()
            .map(|(_, stmt)| stmt.accept_with_context(&Self, context))
            .collect::<Result<Vec<Box<dyn Runnable>>>>()?;
        context.resolver.borrow_mut().exit_scope()?;
        Ok(Box::new(BlockRunnable::new(declarations)))
    }
}
