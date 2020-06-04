extern crate nalgebra;
extern crate num;
#[doc(hidden)]
pub extern crate paste;
extern crate rayon;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate static_assertions;

pub use nalgebra::{DMatrix, DVector};
use num::Float as _;

pub mod drivers;

mod macros;
pub use macros::*;

mod acontext;
pub use acontext::*;

mod afloat;
pub use afloat::*;

#[cfg(feature = "ffi")]
pub mod ffi;

mod float;
pub use float::*;

mod function;
pub use function::*;

mod operation;
pub use operation::*;

mod tape;
pub use tape::*;

/// Default imports that all projects using this crate should have in scope
pub mod prelude {
    pub use super::Function as _;
    pub use super::Tape as _;
    pub use super::TapeExt as _;

    pub use super::Float as _;
    pub use num::Float as _;

    pub use super::adv_dvec;
    pub use super::adv_fn;
    pub use super::adv_fn_obj;
}
