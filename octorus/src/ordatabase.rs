use std::error::Error;

use crate::orresult::ORResult;

pub trait ORDatabase {
    fn new(host: &str, user: &str, password: &str, database: &str) -> Result<Self, Box<dyn Error>> where Self: Sized;
    fn new_with_connection_string(connection_string: &str) -> Result<Self, Box<dyn Error>> where Self: Sized;
    fn close_connection(self: &mut Self) -> Result<(), Box<dyn Error>>;
    fn send_query(
        self: &mut Self,
        query: &str,
        //opts: Option<TOptions>,
    ) -> Result<ORResult, Box<dyn Error>>;
}
