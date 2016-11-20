mod errors;
mod handlers;
pub mod server;

use rustless::{Application, Extensible};
use typemap;

pub struct MatrixNamespace;
impl typemap::Key for MatrixNamespace {
    type Value = String;
}

pub trait MatrixNamespaceExt: Extensible {
    fn matrix_namespace(&self) -> String;
}

impl MatrixNamespaceExt for Application {
    fn matrix_namespace(&self) -> String {
        self.ext().get::<MatrixNamespace>().unwrap().clone()
    }
}
