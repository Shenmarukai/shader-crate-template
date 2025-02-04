//! Shader entry points.
//!
//! Contains an example vertex shader, fragment shader and one example compute
//! shader.
#![no_std]

mod vertex;
mod fragment;

pub use vertex::vertex as vertex;
pub use fragment::fragment as fragment;
