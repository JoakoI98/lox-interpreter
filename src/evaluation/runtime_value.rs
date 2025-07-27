use std::{
    collections::TryReserveError,
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Not, Sub},
};

use thiserror::Error;

use crate::evaluation::{
    evaluator::ClassAccessorError,
    run::{NativeFunctionError, RunScopeRef},
};

#[derive(Clone, Debug)]
pub enum ThisInstance {
    Current(usize),
    WithSuper { current: usize, super_class: usize },
}

impl ThisInstance {
    pub fn get_current(&self) -> usize {
        match self {
            ThisInstance::Current(pointer) => *pointer,
            ThisInstance::WithSuper { current, .. } => *current,
        }
    }

    pub fn get_super_class(&self) -> usize {
        match self {
            ThisInstance::WithSuper { super_class, .. } => *super_class,
            ThisInstance::Current(pointer) => *pointer,
        }
    }

    pub fn current(pointer: usize) -> Self {
        ThisInstance::Current(pointer)
    }

    pub fn remap_to_super(self, current: usize) -> Self {
        let actual_current = self.get_current();
        return Self::WithSuper {
            current: current,
            super_class: actual_current,
        };
    }
}

#[derive(Clone)]
pub enum CallableType {
    Function,
    ClassConstructor,
    Method(ThisInstance),
}

#[derive(Clone)]
pub struct Callable {
    pointer: usize,
    name: String,
    scope: Option<RunScopeRef>,
    ty: CallableType,
}

impl std::fmt::Debug for Callable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let is = match self.ty {
            CallableType::Function => "function",
            CallableType::ClassConstructor => "class",
            CallableType::Method(_) => "method",
        };
        write!(
            f,
            "{} {{ pointer: {}, name: {} }}",
            is, self.pointer, self.name
        )
    }
}

impl Display for Callable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.ty {
            CallableType::Function => write!(f, "<fn {}>", self.name),
            CallableType::ClassConstructor => write!(f, "{}", self.name),
            CallableType::Method(_) => write!(f, "{}", self.name),
        }
    }
}

impl Callable {
    pub fn new(pointer: usize, name: String, scope: Option<RunScopeRef>, ty: CallableType) -> Self {
        Self {
            pointer,
            name,
            scope,
            ty,
        }
    }

    pub fn get_scope(&self) -> Option<RunScopeRef> {
        self.scope.as_ref().map(|scope| scope.clone())
    }

    pub fn get_pointer(&self) -> usize {
        self.pointer
    }

    pub fn get_this_pointer(&self) -> Option<ThisInstance> {
        match &self.ty {
            CallableType::Method(pointer) => Some(pointer.clone()),
            _ => None,
        }
    }

    pub fn map_this_pointer(&mut self, this_pointer: usize) -> () {
        self.ty = match &self.ty {
            CallableType::Method(instance) => {
                CallableType::Method(instance.clone().remap_to_super(this_pointer))
            }
            _ => return,
        };
    }
}

impl PartialEq for Callable {
    fn eq(&self, other: &Self) -> bool {
        self.pointer == other.pointer
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum RuntimeValue {
    Number(f64),
    String(String),
    Boolean(bool),
    Callable(Callable),
    ClassInstance(usize, String),
    Nil,
}

impl Display for RuntimeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeValue::Number(f64) => write!(f, "{}", f64),
            RuntimeValue::String(s) => write!(f, "{}", s),
            RuntimeValue::Boolean(b) => write!(f, "{}", b),
            RuntimeValue::Nil => write!(f, "nil"),
            RuntimeValue::Callable(c) => write!(f, "{}", c),
            RuntimeValue::ClassInstance(_, s) => write!(f, "{} instance", s),
        }
    }
}

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("Operand must be a {0}")]
    UnaryOperandError(String),
    #[error("Operand must be {0}")]
    BinaryOperandError(String),
    #[error("AST invalid structure")]
    ASTInvalidStructure,
    #[error("Undefined variable '{0}'.\nLine: {1}")]
    UndefinedVariable(String, usize),
    #[error("{0}")]
    ResolverError(#[from] super::resolver::ResolverError),
    #[error("{0}")]
    FunctionsResolverError(#[from] super::functions_resolver::FunctionsResolverError),
    #[error("{0}")]
    FunctionEvaluationError(#[from] super::evaluator::FunctionEvaluationError),
    #[error("Arity mismatch")]
    ArityMismatch,
    #[error("{0}")]
    NativeFunctionError(#[from] NativeFunctionError),
    #[error("Function not found")]
    FunctionNotFound,
    #[error("Out of scope")]
    OutOfScope,
    #[error("Not enough space to allocate new scope")]
    NotEnoughSpace(#[from] TryReserveError),
    #[error("Not enough space to allocate new instance")]
    NotEnoughSpaceToAllocateNewInstance,
    #[error("Instance not found")]
    InstanceNotFound(usize),
    #[error("Undefined property '{0}'\n[line {1}]")]
    UndefinedProperty(String, usize),
    #[error("{0}")]
    ClassAccessorError(#[from] ClassAccessorError),
    #[error("This not in scope")]
    ThisNotInScope,
    #[error("Super class not found")]
    SuperClassNotFound,
}

pub type Result<T> = std::result::Result<T, RuntimeError>;

impl Not for RuntimeValue {
    type Output = Result<RuntimeValue>;
    fn not(self) -> Self::Output {
        self.to_bool().map(|b| RuntimeValue::Boolean(!b))
    }
}

impl Neg for RuntimeValue {
    type Output = Result<RuntimeValue>;
    fn neg(self) -> Self::Output {
        match self {
            RuntimeValue::Number(f) => Ok(RuntimeValue::Number(-f)),
            _ => Err(RuntimeError::UnaryOperandError("number".to_string())),
        }
    }
}

impl Mul for RuntimeValue {
    type Output = Result<RuntimeValue>;
    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (RuntimeValue::Number(f1), RuntimeValue::Number(f2)) => {
                Ok(RuntimeValue::Number(f1 * f2))
            }
            _ => Err(RuntimeError::BinaryOperandError("numbers".to_string())),
        }
    }
}

impl Div for RuntimeValue {
    type Output = Result<RuntimeValue>;
    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (RuntimeValue::Number(f1), RuntimeValue::Number(f2)) => {
                Ok(RuntimeValue::Number(f1 / f2))
            }
            _ => Err(RuntimeError::BinaryOperandError("numbers".to_string())),
        }
    }
}

impl Add for RuntimeValue {
    type Output = Result<RuntimeValue>;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (RuntimeValue::Number(f1), RuntimeValue::Number(f2)) => {
                Ok(RuntimeValue::Number(f1 + f2))
            }
            (RuntimeValue::String(s1), RuntimeValue::String(s2)) => {
                Ok(RuntimeValue::String(s1 + s2.as_str()))
            }
            _ => Err(RuntimeError::BinaryOperandError(
                "numbers or strings".to_string(),
            )),
        }
    }
}

impl Sub for RuntimeValue {
    type Output = Result<RuntimeValue>;
    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (RuntimeValue::Number(f1), RuntimeValue::Number(f2)) => {
                Ok(RuntimeValue::Number(f1 - f2))
            }
            _ => Err(RuntimeError::BinaryOperandError(
                "numbers or strings".to_string(),
            )),
        }
    }
}

impl RuntimeValue {
    pub fn callable(
        pointer: usize,
        name: String,
        scope: Option<RunScopeRef>,
        ty: CallableType,
    ) -> Self {
        RuntimeValue::Callable(Callable::new(pointer, name, scope, ty))
    }

    pub fn get_class_instance(&self) -> Option<usize> {
        match self {
            RuntimeValue::ClassInstance(pointer, _) => Some(*pointer),
            _ => None,
        }
    }

    pub fn map_this_pointer(&mut self, this_pointer: usize) -> () {
        match self {
            RuntimeValue::Callable(c) => c.map_this_pointer(this_pointer),
            _ => (),
        }
    }

    pub fn to_bool(&self) -> Result<bool> {
        match self {
            RuntimeValue::Boolean(b) => Ok(*b),
            RuntimeValue::Nil => Ok(false),
            _ => Ok(true),
        }
    }

    pub fn lt(&self, rhs: &RuntimeValue) -> Result<RuntimeValue> {
        match (&self, &rhs) {
            (RuntimeValue::Number(f1), RuntimeValue::Number(f2)) => {
                Ok(RuntimeValue::Boolean(f1 < f2))
            }
            _ => Err(RuntimeError::BinaryOperandError("numbers".to_string())),
        }
    }

    pub fn le(&self, rhs: &RuntimeValue) -> Result<RuntimeValue> {
        match (&self, &rhs) {
            (RuntimeValue::Number(f1), RuntimeValue::Number(f2)) => {
                Ok(RuntimeValue::Boolean(f1 <= f2))
            }
            _ => Err(RuntimeError::BinaryOperandError("numbers".to_string())),
        }
    }

    pub fn gt(&self, rhs: &RuntimeValue) -> Result<RuntimeValue> {
        match (&self, &rhs) {
            (RuntimeValue::Number(f1), RuntimeValue::Number(f2)) => {
                Ok(RuntimeValue::Boolean(f1 > f2))
            }
            _ => Err(RuntimeError::BinaryOperandError("numbers".to_string())),
        }
    }

    pub fn ge(&self, rhs: &RuntimeValue) -> Result<RuntimeValue> {
        match (&self, &rhs) {
            (RuntimeValue::Number(f1), RuntimeValue::Number(f2)) => {
                Ok(RuntimeValue::Boolean(f1 >= f2))
            }
            _ => Err(RuntimeError::BinaryOperandError("numbers".to_string())),
        }
    }

    pub fn eq(&self, rhs: &RuntimeValue) -> Result<RuntimeValue> {
        Ok(RuntimeValue::Boolean(*self == *rhs))
    }

    pub fn ne(&self, rhs: &RuntimeValue) -> Result<RuntimeValue> {
        Ok(RuntimeValue::Boolean(*self != *rhs))
    }
}
