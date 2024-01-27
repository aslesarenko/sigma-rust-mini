use thiserror::Error;

use crate::mir::constant::Constant;
use crate::mir::expr::Expr;
use crate::mir::tuple::Tuple;

use super::PosTrackingWriter;
use super::Printer;

/// Print error
#[allow(missing_docs)]
#[derive(PartialEq, Eq, Debug, Clone, Error)]
pub enum PrintError {
    #[error("fmt error: {0:?}")]
    FmtError(#[from] std::fmt::Error),
}

impl Expr {
    /// Returns pretty printed tree
    pub fn pretty_print(&self) -> Result<(Expr, String), PrintError> {
        let mut printer = PosTrackingWriter::new();
        let spanned_expr = self.print(&mut printer)?;
        let printed_expr_str = printer.get_buf();
        Ok((spanned_expr, printed_expr_str.to_owned()))
    }
}

/// Print trait for Expr that sets the source span for the resulting Expr
pub trait Print {
    /// Print the expression and return the resulting expression with source span
    fn print(&self, w: &mut dyn Printer) -> Result<Expr, PrintError>;
}

impl Print for Expr {
    fn print(&self, w: &mut dyn Printer) -> Result<Expr, PrintError> {
        match self {
            Expr::Const(v) => v.print(w),
            Expr::ConstPlaceholder(_) => Ok(self.clone()),
            Expr::Tuple(v) => v.print(w),
        }
    }
}

impl Print for Constant {
    fn print(&self, w: &mut dyn Printer) -> Result<Expr, PrintError> {
        write!(w, "{:?}", self.v)?;
        Ok(self.clone().into())
    }
}

impl Print for Tuple {
    fn print(&self, w: &mut dyn Printer) -> Result<Expr, PrintError> {
        write!(w, "(")?;
        let items = self.items.try_mapped_ref(|i| {
            write!(w, ", ")?;
            i.print(w)
        })?;
        write!(w, ")")?;
        Ok(Tuple { items }.into())
    }
}

