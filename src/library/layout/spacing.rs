use crate::library::prelude::*;

/// Horizontal spacing.
pub struct HNode;

#[node]
impl HNode {
    fn construct(_: &mut Context, args: &mut Args) -> TypResult<Content> {
        Ok(Content::Horizontal(args.expect("spacing")?))
    }
}

/// Vertical spacing.
pub struct VNode;

#[node]
impl VNode {
    fn construct(_: &mut Context, args: &mut Args) -> TypResult<Content> {
        Ok(Content::Vertical(args.expect("spacing")?))
    }
}

/// Kinds of spacing.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Spacing {
    /// A length stated in absolute values and/or relative to the parent's size.
    Linear(Linear),
    /// A length that is the fraction of the remaining free space in the parent.
    Fractional(Fractional),
}

impl Spacing {
    /// Whether this is fractional spacing.
    pub fn is_fractional(self) -> bool {
        matches!(self, Self::Fractional(_))
    }
}

impl From<Length> for Spacing {
    fn from(length: Length) -> Self {
        Self::Linear(length.into())
    }
}

castable! {
    Spacing,
    Expected: "linear or fractional",
    Value::Length(v) => Self::Linear(v.into()),
    Value::Relative(v) => Self::Linear(v.into()),
    Value::Linear(v) => Self::Linear(v),
    Value::Fractional(v) => Self::Fractional(v),
}
