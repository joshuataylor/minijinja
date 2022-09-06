use std::ops::Deref;

#[cfg(feature = "internal_debug")]
use std::fmt;

use crate::compiler::tokens::Span;
use crate::value::Value;

/// Container for nodes with location info.
///
/// This container fulfills two purposes: it adds location information
/// to nodes, but it also ensures the nodes is heap allocated.  The
/// latter is useful to ensure that enum variants do not cause the enum
/// to become too large.
#[derive(Clone)]
pub struct Spanned<T> {
    node: Box<T>,
    span: Span,
}

impl<T> Spanned<T> {
    /// Creates a new spanned node.
    pub fn new(node: T, span: Span) -> Spanned<T> {
        Spanned {
            node: Box::new(node),
            span,
        }
    }

    /// Accesses the span.
    pub fn span(&self) -> Span {
        self.span
    }
}

impl<T> Deref for Spanned<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.node
    }
}

#[cfg(feature = "internal_debug")]
impl<T: fmt::Debug> fmt::Debug for Spanned<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.node, f)?;
        write!(f, "{:?}", self.span)
    }
}

/// A statement node.
pub enum Stmt<'a> {
    Template(Spanned<Template<'a>>),
    EmitExpr(Spanned<EmitExpr<'a>>),
    EmitRaw(Spanned<EmitRaw<'a>>),
    ForLoop(Spanned<ForLoop<'a>>),
    IfCond(Spanned<IfCond<'a>>),
    WithBlock(Spanned<WithBlock<'a>>),
    Set(Spanned<Set<'a>>),
    SetBlock(Spanned<SetBlock<'a>>),
    Block(Spanned<Block<'a>>),
    Materialization(Spanned<Materialization<'a>>),
    DbtTest(Spanned<DbtTest<'a>>),
    Extends(Spanned<Extends<'a>>),
    Include(Spanned<Include<'a>>),
    AutoEscape(Spanned<AutoEscape<'a>>),
    FilterBlock(Spanned<FilterBlock<'a>>),
    Macro(Spanned<Macro<'a>>),
    Do(Spanned<Do<'a>>),
    MacroCall(Spanned<CallMacroBlock<'a>>),
}

#[cfg(feature = "internal_debug")]
impl<'a> fmt::Debug for Stmt<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Stmt::Template(s) => fmt::Debug::fmt(s, f),
            Stmt::EmitExpr(s) => fmt::Debug::fmt(s, f),
            Stmt::EmitRaw(s) => fmt::Debug::fmt(s, f),
            Stmt::ForLoop(s) => fmt::Debug::fmt(s, f),
            Stmt::IfCond(s) => fmt::Debug::fmt(s, f),
            Stmt::WithBlock(s) => fmt::Debug::fmt(s, f),
            Stmt::Set(s) => fmt::Debug::fmt(s, f),
            Stmt::SetBlock(s) => fmt::Debug::fmt(s, f),
            Stmt::Block(s) => fmt::Debug::fmt(s, f),
            Stmt::Extends(s) => fmt::Debug::fmt(s, f),
            Stmt::Include(s) => fmt::Debug::fmt(s, f),
            Stmt::AutoEscape(s) => fmt::Debug::fmt(s, f),
            Stmt::FilterBlock(s) => fmt::Debug::fmt(s, f),
            Stmt::Macro(s) => fmt::Debug::fmt(s, f),
            Stmt::Do(s) => fmt::Debug::fmt(s, f),
            Stmt::MacroCall(s) => fmt::Debug::fmt(s, f),
            Stmt::Materialization(s) => fmt::Debug::fmt(s, f),
            Stmt::DbtTest(s) => fmt::Debug::fmt(s, f),
        }
    }
}

/// An expression node.
#[allow(clippy::enum_variant_names)]
#[derive(Clone)]
pub enum Expr<'a> {
    Var(Spanned<Var<'a>>),
    Const(Spanned<Const>),
    UnaryOp(Spanned<UnaryOp<'a>>),
    BinOp(Spanned<BinOp<'a>>),
    IfExpr(Spanned<IfExpr<'a>>),
    Filter(Spanned<Filter<'a>>),
    Test(Spanned<Test<'a>>),
    GetAttr(Spanned<GetAttr<'a>>),
    GetItem(Spanned<GetItem<'a>>),
    Call(Spanned<Call<'a>>),
    List(Spanned<List<'a>>),
    Map(Spanned<Map<'a>>),
    Slice(Spanned<Slice<'a>>),
}

/// Applies filters to a block.
#[cfg_attr(feature = "internal_debug", derive(Debug))]
#[derive(Clone)]
pub struct Do<'a> {
    pub target: Expr<'a>,
}

#[cfg_attr(feature = "internal_debug", derive(Debug))]
pub struct CallMacroBlock<'a> {
    pub expr: Expr<'a>,
    pub body: Vec<Stmt<'a>>,
}

#[cfg_attr(feature = "internal_debug", derive(Debug))]
#[derive(Clone)]
pub struct Slice<'a> {
    pub expr: Expr<'a>,
    pub start: Option<Expr<'a>>,
    pub end: Option<Expr<'a>>,
}

#[cfg_attr(feature = "internal_debug", derive(Debug))]
pub struct Macro<'a> {
    pub body: Vec<Stmt<'a>>,
    pub name: &'a str,
    pub args: Vec<(String, Expr<'a>)>,
}

#[cfg(feature = "internal_debug")]
impl<'a> fmt::Debug for Expr<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Var(s) => fmt::Debug::fmt(s, f),
            Expr::Const(s) => fmt::Debug::fmt(s, f),
            Expr::UnaryOp(s) => fmt::Debug::fmt(s, f),
            Expr::BinOp(s) => fmt::Debug::fmt(s, f),
            Expr::IfExpr(s) => fmt::Debug::fmt(s, f),
            Expr::Filter(s) => fmt::Debug::fmt(s, f),
            Expr::Test(s) => fmt::Debug::fmt(s, f),
            Expr::GetAttr(s) => fmt::Debug::fmt(s, f),
            Expr::GetItem(s) => fmt::Debug::fmt(s, f),
            Expr::Call(s) => fmt::Debug::fmt(s, f),
            Expr::List(s) => fmt::Debug::fmt(s, f),
            Expr::Map(s) => fmt::Debug::fmt(s, f),
            Expr::Slice(s) => fmt::Debug::fmt(s, f),
        }
    }
}

/// Root template node.
#[cfg_attr(feature = "internal_debug", derive(Debug))]
pub struct Template<'a> {
    pub children: Vec<Stmt<'a>>,
}

/// A for loop.
#[cfg_attr(feature = "internal_debug", derive(Debug))]
pub struct ForLoop<'a> {
    pub target: Expr<'a>,
    pub iter: Expr<'a>,
    pub filter_expr: Option<Expr<'a>>,
    pub recursive: bool,
    pub body: Vec<Stmt<'a>>,
    pub else_body: Vec<Stmt<'a>>,
}

/// An if/else condition.
#[cfg_attr(feature = "internal_debug", derive(Debug))]
pub struct IfCond<'a> {
    pub expr: Expr<'a>,
    pub true_body: Vec<Stmt<'a>>,
    pub false_body: Vec<Stmt<'a>>,
}

/// A with block.
#[cfg_attr(feature = "internal_debug", derive(Debug))]
pub struct WithBlock<'a> {
    pub assignments: Vec<(Expr<'a>, Expr<'a>)>,
    pub body: Vec<Stmt<'a>>,
}

/// A set statement.
#[cfg_attr(feature = "internal_debug", derive(Debug))]
pub struct Set<'a> {
    pub target: Expr<'a>,
    pub expr: Expr<'a>,
}

/// A set capture statement.
#[cfg_attr(feature = "internal_debug", derive(Debug))]
pub struct SetBlock<'a> {
    pub target: Expr<'a>,
    pub filter: Option<Expr<'a>>,
    pub body: Vec<Stmt<'a>>,
}

/// A block for inheritance elements.
#[cfg_attr(feature = "internal_debug", derive(Debug))]
pub struct Block<'a> {
    pub name: &'a str,
    pub body: Vec<Stmt<'a>>,
}

#[derive(Debug)]
pub struct Materialization<'a> {
    pub name: &'a str,
}

#[derive(Debug)]
pub struct DbtTest<'a> {
    pub name: &'a str,
}

/// An extends block.
#[cfg_attr(feature = "internal_debug", derive(Debug))]
#[derive(Clone)]
pub struct Extends<'a> {
    pub name: Expr<'a>,
}

/// An include block.
#[cfg_attr(feature = "internal_debug", derive(Debug))]
pub struct Include<'a> {
    pub name: Expr<'a>,
    pub ignore_missing: bool,
}

/// An auto escape control block.
#[cfg_attr(feature = "internal_debug", derive(Debug))]
pub struct AutoEscape<'a> {
    pub enabled: Expr<'a>,
    pub body: Vec<Stmt<'a>>,
}

/// Applies filters to a block.
#[cfg_attr(feature = "internal_debug", derive(Debug))]
pub struct FilterBlock<'a> {
    pub filter: Expr<'a>,
    pub body: Vec<Stmt<'a>>,
}

/// Outputs the expression.
#[cfg_attr(feature = "internal_debug", derive(Debug))]
pub struct EmitExpr<'a> {
    pub expr: Expr<'a>,
}

/// Outputs raw template code.
#[cfg_attr(feature = "internal_debug", derive(Debug))]
pub struct EmitRaw<'a> {
    pub raw: &'a str,
}

/// Looks up a variable.
#[cfg_attr(feature = "internal_debug", derive(Debug))]
#[derive(Clone)]
pub struct Var<'a> {
    pub id: &'a str,
}

/// Loads a constant
#[cfg_attr(feature = "internal_debug", derive(Debug))]
#[derive(Clone)]
pub struct Const {
    pub value: Value,
}

/// A kind of unary operator.
#[cfg_attr(feature = "internal_debug", derive(Debug))]
#[derive(Clone)]
pub enum UnaryOpKind {
    Not,
    Neg,
}

/// An unary operator expression.
#[cfg_attr(feature = "internal_debug", derive(Debug))]
#[derive(Clone)]
pub struct UnaryOp<'a> {
    pub op: UnaryOpKind,
    pub expr: Expr<'a>,
}

/// A kind of binary operator.
#[cfg_attr(feature = "internal_debug", derive(Debug))]
#[derive(Clone)]
pub enum BinOpKind {
    Eq,
    Ne,
    Lt,
    Lte,
    Gt,
    Gte,
    ScAnd,
    ScOr,
    Add,
    Sub,
    Mul,
    Div,
    FloorDiv,
    Rem,
    Pow,
    Concat,
    In,
}

/// A binary operator expression.
#[cfg_attr(feature = "internal_debug", derive(Debug))]
#[derive(Clone)]
pub struct BinOp<'a> {
    pub op: BinOpKind,
    pub left: Expr<'a>,
    pub right: Expr<'a>,
}

/// An if expression.
#[cfg_attr(feature = "internal_debug", derive(Debug))]
#[derive(Clone)]
pub struct IfExpr<'a> {
    pub test_expr: Expr<'a>,
    pub true_expr: Expr<'a>,
    pub false_expr: Option<Expr<'a>>,
}

/// A filter expression.
#[cfg_attr(feature = "internal_debug", derive(Debug))]
#[derive(Clone)]
pub struct Filter<'a> {
    pub name: &'a str,
    pub expr: Option<Expr<'a>>,
    pub args: Vec<Expr<'a>>,
}

/// A test expression.
#[cfg_attr(feature = "internal_debug", derive(Debug))]
#[derive(Clone)]
pub struct Test<'a> {
    pub name: &'a str,
    pub expr: Expr<'a>,
    pub args: Vec<Expr<'a>>,
}

/// An attribute lookup expression.
#[cfg_attr(feature = "internal_debug", derive(Debug))]
#[derive(Clone)]
pub struct GetAttr<'a> {
    pub expr: Expr<'a>,
    pub name: &'a str,
}

/// An item lookup expression.
#[cfg_attr(feature = "internal_debug", derive(Debug))]
#[derive(Clone)]
pub struct GetItem<'a> {
    pub expr: Expr<'a>,
    pub subscript_expr: Expr<'a>,
}

/// Calls something.
#[cfg_attr(feature = "internal_debug", derive(Debug))]
#[derive(Clone)]
pub struct Call<'a> {
    pub expr: Expr<'a>,
    pub args: Vec<Expr<'a>>,
}

/// Creates a list of values.
#[cfg_attr(feature = "internal_debug", derive(Debug))]
#[derive(Clone)]
pub struct List<'a> {
    pub items: Vec<Expr<'a>>,
}

/// Creates a map of values.
#[cfg_attr(feature = "internal_debug", derive(Debug))]
#[derive(Clone)]
pub struct Map<'a> {
    pub keys: Vec<Expr<'a>>,
    pub values: Vec<Expr<'a>>,
}

/// Defines the specific type of call.
#[cfg_attr(feature = "internal_debug", derive(Debug))]
pub enum CallType<'ast, 'source> {
    Function(&'source str),
    Method(&'ast Expr<'source>, &'source str),
    Block(&'source str),
    Object(&'ast Expr<'source>),
}

impl<'a> Call<'a> {
    /// Try to isolate a method call.
    ///
    /// name + call and attribute lookup + call are really method
    /// calls which are easier to handle for the compiler as a separate
    /// thing.
    pub fn identify_call(&self) -> CallType<'_, 'a> {
        match self.expr {
            Expr::Var(ref var) => CallType::Function(var.id),
            Expr::GetAttr(ref attr) => {
                if let Expr::Var(ref var) = attr.expr {
                    if var.id == "self" {
                        return CallType::Block(attr.name);
                    }
                }
                CallType::Method(&attr.expr, attr.name)
            }
            _ => CallType::Object(&self.expr),
        }
    }
}
