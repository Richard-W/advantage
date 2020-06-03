use super::*;
use std::fmt;

/// Type behaving like a floating-point number
pub trait Float: num::Float + fmt::Debug + Send + Sync + 'static {}

impl<T> Float for T where T: num::Float + fmt::Debug + Send + Sync + 'static {}
assert_impl_all!(f64: Float);
assert_impl_all!(ADouble: Float);
