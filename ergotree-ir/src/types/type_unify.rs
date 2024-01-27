use std::collections::HashMap;

use super::stype::SType;
use super::stype_param::STypeVar;
use SType::*;

#[allow(clippy::unnecessary_wraps)]
fn unified_without_subst() -> Result<HashMap<STypeVar, SType>, TypeUnificationError> {
    Ok(HashMap::new())
}

/// Performs pairwise type unification making sure each type variable is equally
/// substituted in all items.
pub fn unify_many(
    items1: Vec<SType>,
    items2: Vec<SType>,
) -> Result<HashMap<STypeVar, SType>, TypeUnificationError> {
    if items1.len() != items2.len() {
        return Err(TypeUnificationError(format!(
            "items lists are different sizes {:?} vs. {:?}",
            items1, items2
        )));
    }
    let list_of_substitutions: Result<Vec<HashMap<STypeVar, SType>>, _> = items1
        .iter()
        .zip(items2)
        .map(|(t1, t2)| unify_one(t1, &t2))
        .collect();
    let mut res = HashMap::new();
    for substitutions in list_of_substitutions? {
        for (type_var, tpe) in substitutions {
            match res.insert(type_var.clone(), tpe.clone()) {
                Some(previous_val) if previous_val != tpe => {
                    return Err(TypeUnificationError(format!(
                        "cannot merge new substitution {:?} for {:?} already exist substitution {:?}",
                        tpe, type_var, previous_val
                    )))
                }
                _ => (),
            };
        }
    }
    Ok(res)
}

/// Error on type unification
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct TypeUnificationError(pub String);

/// Finds a substitution `subst` of type variables
/// such that unify_types(t1.with_subst(subst), t2) == Ok(emptySubst)
pub fn unify_one(t1: &SType, t2: &SType) -> Result<HashMap<STypeVar, SType>, TypeUnificationError> {
    match (t1, t2) {
        (STypeVar(tv1), STypeVar(tv2)) if tv1 == tv2 => unified_without_subst(),
        (STypeVar(id1), t2) if !matches!(t2, STypeVar(_)) => {
            Ok([(id1.clone(), t2.clone())].iter().cloned().collect())
        }
        (t1, t2) if t1.is_prim() && t2.is_prim() && t1 == t2 => unified_without_subst(),
        (SColl(elem_type1), SColl(elem_type2)) => unify_one(elem_type1, elem_type2),
        (SColl(elem_type1), STuple(_)) => unify_one(elem_type1, &SAny),
        (STuple(tuple1), STuple(tuple2)) if tuple1.items.len() == tuple2.items.len() => {
            unify_many(tuple1.items.clone().into(), tuple2.items.clone().into())
        }
        (SOption(elem_type1), SOption(elem_type2)) => unify_one(elem_type1, elem_type2),
        (SAny, _) => unified_without_subst(),
        // it is necessary for implicit conversion in Coll(bool, prop, bool)
        (SBoolean, SSigmaProp) => unified_without_subst(),
        (t1, t2) => Err(TypeUnificationError(format!(
            "Cannot unify {:?} and {:?}",
            t1, t2
        ))),
    }
}

#[allow(clippy::unwrap_used)]
#[cfg(test)]
#[allow(clippy::panic)]
mod tests {
    use super::super::stype::tests::primitive_type;
    use super::*;
    use crate::types::stuple::STuple;
    use proptest::prelude::*;

    proptest! {

        #[test]
        fn prim_types(t in primitive_type()) {
            prop_assert_eq!(unify_one(&t, &t), unified_without_subst());
            prop_assert_eq!(unify_one(&SAny, &t), unified_without_subst());
            prop_assert_eq!(unify_one(&SAny, &SColl(t.clone().into())), unified_without_subst());
            prop_assert_eq!(unify_one(&SColl(SAny.into()), &SColl(t.clone().into())), unified_without_subst());
            prop_assert_eq!(unify_one(
                &SColl(SAny.into()),
                &STuple(STuple::pair(t.clone(), t.clone()))), unified_without_subst()
            );
            prop_assert_eq!(unify_one(
                &SColl(SAny.into()),
                &STuple(STuple::pair(t.clone(), STuple(STuple::pair(t.clone(), t))))), unified_without_subst()
            );
        }

    }

    fn check_error(t1: SType, t2: SType) {
        assert!(
            unify_one(&t1, &t2).is_err(),
            "unification of {:?} and {:?} should fail",
            t1,
            t2
        );
    }

    #[test]
    fn unify_negative() {
        check_error(SInt, SLong);

        // Tuple
        check_error(SInt, STuple::pair(SInt, SBoolean).into());
        check_error(
            STuple::pair(SBoolean, SInt).into(),
            STuple::pair(SBoolean, SBoolean).into(),
        );
        check_error(STuple::pair(SInt, STypeVar::t().into()).into(), SInt);
        check_error(
            STuple::pair(STypeVar::t().into(), SInt).into(),
            STuple::pair(SBoolean, STypeVar::iv().into()).into(),
        );

        // Coll
        check_error(SColl(SColl(SInt.into()).into()), SColl(SInt.into()));
        check_error(
            SColl(Box::new(STypeVar::t().into())),
            SColl(Box::new(STypeVar::iv().into())),
        );

        // Option
        check_error(SOption(SBoolean.into()), SOption(SInt.into()));
        check_error(SOption(SBoolean.into()), SColl(SInt.into()));

        check_error(SSigmaProp, SBoolean);
    }

    #[test]
    fn unify_diff_size() {
        assert!(unify_many(vec![SInt, SLong], vec![SLong]).is_err());
    }
}
