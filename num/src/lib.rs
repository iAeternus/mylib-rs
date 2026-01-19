pub mod core;
pub use crate::core::*;

#[cfg(not(feature = "core"))]
pub mod big_num;
#[cfg(not(feature = "core"))]
pub mod complex;
#[cfg(not(feature = "core"))]
pub mod error;
#[cfg(not(feature = "core"))]
pub mod frac;
#[cfg(not(feature = "core"))]
pub mod impls;
#[cfg(not(feature = "core"))]
pub mod matrix;
#[cfg(not(feature = "core"))]
pub mod vector;
