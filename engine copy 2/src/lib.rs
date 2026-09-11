use std::sync::OnceLock;

mod context;
pub use context::*;

mod units;
pub use units::*;

pub mod logger;

mod ssf;
pub use ssf::*;

pub fn ui() -> egui::Context {
    static CTX: OnceLock<egui::Context> = OnceLock::new();
    CTX.get_or_init(|| egui::Context::default()).clone()
}