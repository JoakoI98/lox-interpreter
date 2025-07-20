mod basic_runners;
mod class_runners;
mod control_flow_runners;
mod declaration_runners;
mod function_runners;

// Re-export the Runnable trait
pub use basic_runners::Runnable;

// Re-export all runner implementations
pub use basic_runners::{
    ExpressionRunnable, FunctionDeclarationRunnable, PrintRunnable, ProgramRunnable, ReturnRunnable,
};
pub use class_runners::{ClassDeclarationRunnable, ClassInitializationCallable};
pub use control_flow_runners::{ForStatementRunnable, IsStatementRunnable, WhileStatementRunnable};
pub use declaration_runners::{BlockRunnable, VarDeclarationRunnable};
pub use function_runners::{get_native_functions, Callable, NativeFunctionError};
