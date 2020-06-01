use std::fmt;

pub trait Float: num::Float + fmt::Debug + Send + Sync + 'static {}

impl<T> Float for T where T: num::Float + fmt::Debug + Send + Sync + 'static {}
