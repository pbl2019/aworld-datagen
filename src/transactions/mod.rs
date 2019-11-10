pub mod forward;

use crate::connection::Connection;
use crate::context::Context;

pub trait Transaction {
    fn call(&self, conn: &Connection, context: &Context) -> Result<Vec<i64>, ()>;
}
