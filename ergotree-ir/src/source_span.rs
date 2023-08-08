//! Source position for an IR node in the source code

use crate::mir::bin_op::BinOp;
use crate::mir::block::BlockValue;
use crate::mir::coll_append::Append;
use crate::mir::expr::Expr;
use crate::mir::val_def::ValDef;

/// Source position for the Expr
#[derive(PartialEq, Eq, Debug, Clone)]
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

macro_rules! into_expr {
    ($variant: ident) => {
        impl From<$variant> for Expr {
            fn from(v: $variant) -> Self {
                Expr::$variant(Spanned {
                    source_span: SourceSpan::empty(),
                    expr: v,
                })
            }
        }
    };
}

into_expr!(Append);
into_expr!(BlockValue);
into_expr!(ValDef);
into_expr!(BinOp);

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
    #[allow(clippy::todo)]
    pub fn span(&self) -> &SourceSpan {
        match self {
            Expr::Append(op) => &op.source_span,
            Expr::Const(_) => todo!(),
            Expr::ConstPlaceholder(_) => todo!(),
            Expr::SubstConstants(_) => todo!(),
            Expr::ByteArrayToLong(_) => todo!(),
            Expr::ByteArrayToBigInt(_) => todo!(),
            Expr::LongToByteArray(_) => todo!(),
            Expr::Collection(_) => todo!(),
            Expr::Tuple(_) => todo!(),
            Expr::CalcBlake2b256(_) => todo!(),
            Expr::CalcSha256(_) => todo!(),
            Expr::Context => todo!(),
            Expr::Global => todo!(),
            Expr::GlobalVars(_) => todo!(),
            Expr::FuncValue(_) => todo!(),
            Expr::Apply(_) => todo!(),
            Expr::MethodCall(_) => todo!(),
            Expr::ProperyCall(_) => todo!(),
            Expr::BlockValue(_) => todo!(),
            Expr::ValDef(_) => todo!(),
            Expr::ValUse(_) => todo!(),
            Expr::If(_) => todo!(),
            Expr::BinOp(_) => todo!(),
            Expr::And(_) => todo!(),
            Expr::Or(_) => todo!(),
            Expr::Xor(_) => todo!(),
            Expr::Atleast(_) => todo!(),
            Expr::LogicalNot(_) => todo!(),
            Expr::Negation(_) => todo!(),
            Expr::BitInversion(_) => todo!(),
            Expr::OptionGet(_) => todo!(),
            Expr::OptionIsDefined(_) => todo!(),
            Expr::OptionGetOrElse(_) => todo!(),
            Expr::ExtractAmount(_) => todo!(),
            Expr::ExtractRegisterAs(_) => todo!(),
            Expr::ExtractBytes(_) => todo!(),
            Expr::ExtractBytesWithNoRef(_) => todo!(),
            Expr::ExtractScriptBytes(_) => todo!(),
            Expr::ExtractCreationInfo(_) => todo!(),
            Expr::ExtractId(_) => todo!(),
            Expr::ByIndex(_) => todo!(),
            Expr::SizeOf(_) => todo!(),
            Expr::Slice(_) => todo!(),
            Expr::Fold(_) => todo!(),
            Expr::Map(_) => todo!(),
            Expr::Filter(_) => todo!(),
            Expr::Exists(_) => todo!(),
            Expr::ForAll(_) => todo!(),
            Expr::SelectField(_) => todo!(),
            Expr::BoolToSigmaProp(_) => todo!(),
            Expr::Upcast(_) => todo!(),
            Expr::Downcast(_) => todo!(),
            Expr::CreateProveDlog(_) => todo!(),
            Expr::CreateProveDhTuple(_) => todo!(),
            Expr::SigmaPropBytes(_) => todo!(),
            Expr::DecodePoint(_) => todo!(),
            Expr::SigmaAnd(_) => todo!(),
            Expr::SigmaOr(_) => todo!(),
            Expr::GetVar(_) => todo!(),
            Expr::DeserializeRegister(_) => todo!(),
            Expr::DeserializeContext(_) => todo!(),
            Expr::MultiplyGroup(_) => todo!(),
            Expr::Exponentiate(_) => todo!(),
            Expr::XorOf(_) => todo!(),
            Expr::TreeLookup(_) => todo!(),
            Expr::CreateAvlTree(_) => todo!(),
        }
    }
}
