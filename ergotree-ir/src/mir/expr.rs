//! IR expression

use std::convert::TryFrom;
use std::convert::TryInto;

use crate::types::stype::LiftIntoSType;
use crate::types::stype::SType;

use super::collection::Collection;
use super::constant::Constant;
use super::constant::ConstantPlaceholder;
use super::constant::Literal;
use super::constant::TryExtractFrom;
use super::constant::TryExtractFromError;
use super::tuple::Tuple;

extern crate derive_more;
use bounded_vec::BoundedVecOutOfBounds;
use derive_more::From;
use derive_more::TryInto;
use thiserror::Error;

#[derive(PartialEq, Eq, Debug, Clone, From, TryInto)]
/// Expression in ErgoTree
pub enum Expr {
    /// Constant value
    Const(Constant),
    /// Placeholder for a constant
    ConstPlaceholder(ConstantPlaceholder),
    /// Collection declaration (array of expressions of the same type)
    Collection(Collection),
    /// Tuple declaration
    Tuple(Tuple),
}

impl Expr {
    /// Type of the expression
    pub fn tpe(&self) -> SType {
        match self {
            Expr::Const(v) => v.tpe.clone(),
            Expr::Collection(v) => v.tpe(),
            Expr::ConstPlaceholder(v) => v.tpe.clone(),
            Expr::Tuple(v) => v.tpe(),
        }
    }

    /// Type expected after the evaluation
    pub fn post_eval_tpe(&self) -> SType {
        match self.tpe() {
            SType::SFunc(sfunc) => *sfunc.t_range,
            tpe => tpe,
        }
    }

    /// Check if given expected_tpe type is the same as the expression's post-evaluation type
    pub fn check_post_eval_tpe(
        &self,
        expected_tpe: &SType,
    ) -> Result<(), InvalidExprEvalTypeError> {
        let expr_tpe = self.post_eval_tpe();
        if &expr_tpe == expected_tpe {
            Ok(())
        } else {
            use std::backtrace::Backtrace;
            let backtrace = Backtrace::capture();
            Err(InvalidExprEvalTypeError(format!(
                "expected: {0:?}, got: {1:?}\nBacktrace:\n{backtrace}",
                expected_tpe, expr_tpe
            )))
        }
    }

    /// Prints the tree with newlines
    pub fn debug_tree(&self) -> String {
        let tree = format!("{:#?}", self);
        tree
    }
}

impl<T: Into<Literal> + LiftIntoSType> From<T> for Expr {
    fn from(t: T) -> Self {
        Expr::Const(Constant {
            tpe: T::stype(),
            v: t.into(),
        })
    }
}

/// Unexpected argument on node construction (i.e non-Option input in OptionGet)
#[derive(Error, PartialEq, Eq, Debug, Clone)]
#[error("InvalidArgumentError: {0}")]
pub struct InvalidArgumentError(pub String);

/// Invalid (unexpected) expr type
#[derive(PartialEq, Eq, Debug, Clone, Error)]
#[error("InvalidExprEvalTypeError: {0}")]
pub struct InvalidExprEvalTypeError(pub String);

impl From<InvalidExprEvalTypeError> for InvalidArgumentError {
    fn from(e: InvalidExprEvalTypeError) -> Self {
        InvalidArgumentError(format!("InvalidExprEvalTypeError: {0}", e))
    }
}

impl From<BoundedVecOutOfBounds> for InvalidArgumentError {
    fn from(e: BoundedVecOutOfBounds) -> Self {
        InvalidArgumentError(format!("BoundedVecOutOfBounds: {0}", e))
    }
}

impl<T: TryFrom<Expr>> TryExtractFrom<Expr> for T {
    fn try_extract_from(v: Expr) -> Result<Self, TryExtractFromError> {
        let res: Result<Self, TryExtractFromError> = v.clone().try_into().map_err(|_| {
            TryExtractFromError(format!(
                "Cannot extract {0:?} from {1:?}",
                std::any::type_name::<T>(),
                v
            ))
        });
        res
    }
}

#[cfg(feature = "arbitrary")]
#[allow(clippy::unwrap_used)]
#[allow(clippy::panic)]
#[allow(clippy::todo)]
/// Arbitrary impl
pub(crate) mod arbitrary {
    use super::*;
    use proptest::prelude::*;

    /// Parameters for arbitrary Expr generation
    #[derive(PartialEq, Eq, Debug, Clone)]
    pub struct ArbExprParams {
        /// Expr type
        pub tpe: SType,
        /// Expr tree depth (levels)
        pub depth: usize,
    }

    impl Default for ArbExprParams {
        fn default() -> Self {
            ArbExprParams {
                tpe: SType::SBoolean,
                depth: 1,
            }
        }
    }

    fn constant(tpe: &SType) -> BoxedStrategy<Expr> {
        any_with::<Constant>(tpe.clone().into())
            .prop_map_into()
            .boxed()
    }

    fn bool_non_nested_expr() -> BoxedStrategy<Expr> {
        prop_oneof![any_with::<Constant>(SType::SBoolean.into()).prop_map_into()].boxed()
    }

    fn any_non_nested_expr() -> BoxedStrategy<Expr> {
        prop_oneof![bool_non_nested_expr()].boxed()
    }

    fn coll_non_nested_expr(elem_tpe: &SType) -> BoxedStrategy<Expr> {
        match elem_tpe {
            SType::SBoolean => any_with::<Constant>(SType::SColl(Box::new(SType::SBoolean)).into())
                .prop_map(Expr::Const)
                .boxed(),
            SType::SByte => any_with::<Constant>(SType::SColl(Box::new(SType::SByte)).into())
                .prop_map(Expr::Const)
                .boxed(),
            SType::SShort => any_with::<Constant>(SType::SColl(Box::new(SType::SShort)).into())
                .prop_map(Expr::Const)
                .boxed(),
            SType::SInt => any_with::<Constant>(SType::SColl(Box::new(SType::SInt)).into())
                .prop_map(Expr::Const)
                .boxed(),
            SType::SLong => any_with::<Constant>(SType::SColl(Box::new(SType::SLong)).into())
                .prop_map(Expr::Const)
                .boxed(),
            _ => todo!("Collection of {0:?} is not yet implemented", elem_tpe),
        }
    }

    fn non_nested_expr(tpe: &SType) -> BoxedStrategy<Expr> {
        match tpe {
            SType::SAny => any_non_nested_expr(),
            SType::SBoolean => bool_non_nested_expr(),
            SType::SColl(elem_type) => coll_non_nested_expr(elem_type),
            t => constant(t),
        }
    }

    impl Arbitrary for Expr {
        type Parameters = ArbExprParams;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(args: Self::Parameters) -> Self::Strategy {
            #[allow(clippy::match_single_binding)]
            match args.tpe {
                _ => prop_oneof![
                    any_with::<Constant>(args.tpe.clone().into())
                        .prop_map(Expr::Const)
                        .boxed(),
                    non_nested_expr(&args.tpe)
                ]
                .boxed(),
            }
        }
    }
}

#[cfg(test)]
mod tests {}
