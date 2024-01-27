//! Source position for an IR node in the source code

use crate::mir::expr::Expr;

/// Source position for the Expr
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct SourceSpan {
    /// Start position in the span
    pub offset: usize,
    /// The length of the span
    pub length: usize,
}

impl SourceSpan {
    /// Empty span
    pub fn empty() -> Self {
        SourceSpan {
            offset: 0,
            length: 0,
        }
    }
}

impl From<(usize, usize)> for SourceSpan {
    fn from(value: (usize, usize)) -> Self {
        SourceSpan {
            offset: value.0,
            length: value.1,
        }
    }
}

impl From<SourceSpan> for miette::SourceSpan {
    fn from(value: SourceSpan) -> Self {
        miette::SourceSpan::new(value.offset.into(), value.length.into())
    }
}

/// Wrapper for Expr with source position
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Spanned<T> {
    /// Source position
    pub source_span: SourceSpan,
    /// Wrapped value
    pub expr: T,
}

impl<T> Spanned<T> {
    /// Expression
    pub fn expr(&self) -> &T {
        &self.expr
    }
}

impl<T> From<T> for Spanned<T> {
    fn from(v: T) -> Self {
        Spanned {
            source_span: SourceSpan::empty(),
            expr: v,
        }
    }
}

impl Expr {
    /// Source span for the Expr
    pub fn span(&self) -> SourceSpan {
        match self {
            Expr::Const(_) => SourceSpan::empty(),
            Expr::ConstPlaceholder(_) => SourceSpan::empty(),
            Expr::Tuple(_) => SourceSpan::empty(),
        }
    }
}
