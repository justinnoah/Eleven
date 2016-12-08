mod errors;
mod handlers;
pub mod server;

use rusqlite::Connection;
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


pub struct DB;
impl typemap::Key for DB {
    type Value = Connection;
}

pub trait DBExt: Extensible {
    fn db_con(&self) -> &Connection;
}
impl DBExt for Application {
    fn db_con(&self) -> &Connection {
        self.ext().get::<DB>().unwrap()
    }
}
