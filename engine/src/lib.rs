pub mod logger;

pub mod json_helper;

mod context;
pub use context::*;

mod texture;
pub use texture::*;

mod material;
pub use material::*;

mod ubo;
pub use ubo::*;

mod rendertexture;
pub use rendertexture::*;

mod mesh;
pub use mesh::*;

mod bind_group;
pub use bind_group::*;

use std::sync::OnceLock;

pub fn ui() -> egui::Context {
    static CTX: OnceLock<egui::Context> = OnceLock::new();
    CTX.get_or_init(|| egui::Context::default()).clone()
}