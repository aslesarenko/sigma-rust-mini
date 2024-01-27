use super::stype::SType;
use super::stype_param::STypeParam;

/// Function signature type
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SFunc {
    /// Function parameter types
    pub t_dom: Vec<SType>,
    /// Result type
    pub t_range: Box<SType>,
    /// Type parameters if the function is generic
    pub tpe_params: Vec<STypeParam>,
}

impl std::fmt::Display for SFunc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(")?;
        for (i, item) in self.t_dom.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            item.fmt(f)?;
        }
        write!(f, ") => ")?;
        self.t_range.fmt(f)
    }
}

impl SFunc {
    /// Create new SFunc
    pub fn new(t_dom: Vec<SType>, t_range: SType) -> Self {
        Self {
            t_dom,
            t_range: t_range.into(),
            tpe_params: vec![],
        }
    }

    /// Returns function parameter types (t_dom) with added result type (t_range)
    pub fn t_dom_plus_range(&self) -> Vec<SType> {
        let mut res = self.t_dom.clone();
        res.push(*self.t_range.clone());
        res
    }
}
