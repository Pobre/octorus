use mongodb::Client;

#[derive(Debug)]
pub struct ORMongoDB {
    client: Client,
}
