//! Source position for an IR node in the source code

use crate::mir::and::And;
use crate::mir::bin_op::BinOp;
use crate::mir::block::BlockValue;
use crate::mir::byte_array_to_bigint::ByteArrayToBigInt;
use crate::mir::byte_array_to_long::ByteArrayToLong;
use crate::mir::coll_append::Append;
use crate::mir::coll_by_index::ByIndex;
use crate::mir::coll_exists::Exists;
use crate::mir::coll_filter::Filter;
use crate::mir::coll_fold::Fold;
use crate::mir::coll_forall::ForAll;
use crate::mir::coll_map::Map;
use crate::mir::coll_slice::Slice;
use crate::mir::expr::Expr;
use crate::mir::extract_reg_as::ExtractRegisterAs;
use crate::mir::get_var::GetVar;
use crate::mir::logical_not::LogicalNot;
use crate::mir::method_call::MethodCall;
use crate::mir::negation::Negation;
use crate::mir::option_get::OptionGet;
use crate::mir::option_get_or_else::OptionGetOrElse;
use crate::mir::option_is_defined::OptionIsDefined;
use crate::mir::or::Or;
use crate::mir::property_call::PropertyCall;
use crate::mir::select_field::SelectField;
use crate::mir::subst_const::SubstConstants;
use crate::mir::tree_lookup::TreeLookup;
use crate::mir::val_def::ValDef;

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
into_expr!(ByIndex);
into_expr!(SubstConstants);
into_expr!(ByteArrayToLong);
into_expr!(ByteArrayToBigInt);
into_expr!(MethodCall);
into_expr!(PropertyCall);
into_expr!(Negation);
into_expr!(OptionGet);
into_expr!(OptionIsDefined);
into_expr!(OptionGetOrElse);
into_expr!(ExtractRegisterAs);
into_expr!(Slice);
into_expr!(Fold);
into_expr!(Map);
into_expr!(Filter);
into_expr!(Exists);
into_expr!(ForAll);
into_expr!(SelectField);
into_expr!(GetVar);
into_expr!(TreeLookup);
into_expr!(And);
into_expr!(Or);
into_expr!(LogicalNot);

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
            Expr::Append(op) => op.source_span,
            Expr::Const(_) => SourceSpan::empty(),
            Expr::ConstPlaceholder(_) => SourceSpan::empty(),
            Expr::SubstConstants(op) => op.source_span,
            Expr::ByteArrayToLong(op) => op.source_span,
            Expr::ByteArrayToBigInt(op) => op.source_span,
            Expr::LongToByteArray(_) => SourceSpan::empty(),
            Expr::Collection(_) => SourceSpan::empty(),
            Expr::Tuple(_) => SourceSpan::empty(),
            Expr::CalcBlake2b256(_) => SourceSpan::empty(),
            Expr::CalcSha256(_) => SourceSpan::empty(),
            Expr::Context => SourceSpan::empty(),
            Expr::Global => SourceSpan::empty(),
            Expr::GlobalVars(_) => SourceSpan::empty(),
            Expr::FuncValue(_) => SourceSpan::empty(),
            Expr::Apply(_) => SourceSpan::empty(),
            Expr::MethodCall(op) => op.source_span,
            Expr::PropertyCall(op) => op.source_span,
            Expr::BlockValue(op) => op.source_span,
            Expr::ValDef(op) => op.source_span,
            Expr::ValUse(_) => SourceSpan::empty(),
            Expr::If(_) => SourceSpan::empty(),
            Expr::BinOp(op) => op.source_span,
            Expr::And(_) => SourceSpan::empty(),
            Expr::Or(_) => SourceSpan::empty(),
            Expr::Xor(_) => SourceSpan::empty(),
            Expr::Atleast(_) => SourceSpan::empty(),
            Expr::LogicalNot(_) => SourceSpan::empty(),
            Expr::Negation(op) => op.source_span,
            Expr::BitInversion(_) => SourceSpan::empty(),
            Expr::OptionGet(op) => op.source_span,
            Expr::OptionIsDefined(op) => op.source_span,
            Expr::OptionGetOrElse(op) => op.source_span,
            Expr::ExtractAmount(_) => SourceSpan::empty(),
            Expr::ExtractRegisterAs(op) => op.source_span,
            Expr::ExtractBytes(_) => SourceSpan::empty(),
            Expr::ExtractBytesWithNoRef(_) => SourceSpan::empty(),
            Expr::ExtractScriptBytes(_) => SourceSpan::empty(),
            Expr::ExtractCreationInfo(_) => SourceSpan::empty(),
            Expr::ExtractId(_) => SourceSpan::empty(),
            Expr::ByIndex(op) => op.source_span,
            Expr::SizeOf(_) => SourceSpan::empty(),
            Expr::Slice(op) => op.source_span,
            Expr::Fold(op) => op.source_span,
            Expr::Map(op) => op.source_span,
            Expr::Filter(op) => op.source_span,
            Expr::Exists(op) => op.source_span,
            Expr::ForAll(op) => op.source_span,
            Expr::SelectField(op) => op.source_span,
            Expr::BoolToSigmaProp(_) => SourceSpan::empty(),
            Expr::Upcast(_) => SourceSpan::empty(),
            Expr::Downcast(_) => SourceSpan::empty(),
            Expr::CreateProveDlog(_) => SourceSpan::empty(),
            Expr::CreateProveDhTuple(_) => SourceSpan::empty(),
            Expr::SigmaPropBytes(_) => SourceSpan::empty(),
            Expr::DecodePoint(_) => SourceSpan::empty(),
            Expr::SigmaAnd(_) => SourceSpan::empty(),
            Expr::SigmaOr(_) => SourceSpan::empty(),
            Expr::GetVar(op) => op.source_span,
            Expr::DeserializeRegister(_) => SourceSpan::empty(),
            Expr::DeserializeContext(_) => SourceSpan::empty(),
            Expr::MultiplyGroup(_) => SourceSpan::empty(),
            Expr::Exponentiate(_) => SourceSpan::empty(),
            Expr::XorOf(_) => SourceSpan::empty(),
            Expr::TreeLookup(op) => op.source_span,
        }
    }
}
