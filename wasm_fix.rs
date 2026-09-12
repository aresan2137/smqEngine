// fix for winit droped_file

use std::path::{Path, PathBuf};
use std::pin::Pin;

#[derive(Debug)]
pub(crate) struct NativeFile {
    path: PathBuf,
}

impl From<PathBuf> for NativeFile {
    fn from(path: PathBuf) -> Self {
        Self { path }
    }
}

impl egui::DroppedFile for NativeFile {
    fn path(&self) -> &Path {
        &self.path
    }

    fn bytes_async(&self) -> Pin<Box<dyn Future<Output = Result<std::vec::Vec<u8>, String>>>> { todo!() }
}
