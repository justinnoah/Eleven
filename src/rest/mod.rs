mod errors;
mod endpoints;
pub mod server;

use r2d2;
use r2d2_sqlite::SqliteConnectionManager;
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
    type Value = r2d2::Pool<SqliteConnectionManager>;
}
pub trait DBExt: Extensible {
    fn db_con(&self) -> r2d2::Pool<SqliteConnectionManager>;
}
impl DBExt for Application {
    fn db_con(&self) -> r2d2::Pool<SqliteConnectionManager> {
        self.ext().get::<DB>().unwrap().clone()
    }
}
