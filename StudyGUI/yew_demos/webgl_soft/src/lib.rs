#[macro_use]
extern crate lazy_static;

extern crate nalgebra as na;

pub mod random_color;
pub use random_color::*;

pub mod glsl;
pub use glsl::*;

pub mod model_obj;
pub use model_obj::*;

pub mod model;
pub use model::*;

pub mod mesh;
pub use mesh::*;

pub mod renderer;
pub use renderer::*;

pub mod renderer_soft;
pub use renderer_soft::*;
