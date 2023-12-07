use async_trait::async_trait;
use std::error::Error;

#[async_trait]
pub trait ORDatabase {
    async fn new(
        host: &str,
        user: &str,
        password: &str,
        database: &str,
    ) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized;
    async fn new_with_connection_string(connection_string: &str) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized;
    async fn close_connection(self: &mut Self) -> Result<(), Box<dyn Error>>;
}
