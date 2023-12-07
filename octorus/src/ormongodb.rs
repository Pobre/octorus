use std::{collections::HashMap, error::Error};

use async_trait::async_trait;
use bson::doc;

use mongodb::{
    options::{ClientOptions, ConnectionString},
    Client, Collection, Database,
};
use serde_json::Value;

use crate::{ordatabase::{ORDatabase}, orresult::ORResult};

#[derive(Debug)]
pub struct ORMongoDB {
    client: Client,
    db: Database,
    collection: String,
}

impl ORMongoDB {
    fn new(client: Client) -> Self {
        let db = client.default_database().unwrap();
        Self {
            client,
            db,
            collection: String::new(),
        }
    }
    pub async fn find_one(self: &mut Self, query: &str) -> Result<ORResult, Box<dyn Error>> {
        let _map = serde_json::from_slice::<HashMap<String, Value>>(query.as_bytes()).unwrap();
        todo!()
    }
}

#[async_trait]
impl ORDatabase for ORMongoDB {
    async fn new(
        host: &str,
        user: &str,
        password: &str,
        database: &str,
    ) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized,
    {
        let uri = format!("mongodb://{0}:{1}@{2}/{3}", user, password, host, database);
        let cs = ConnectionString::parse(uri).expect("nao é uri valido");
        let options = ClientOptions::parse_connection_string(cs)
            .await
            .expect("nao deu parse em uri");
        let client = Client::with_options(options).expect("nao conectou");
        Ok(ORMongoDB::new(client))
    }

    async fn new_with_connection_string(
        connection_string: &str,
    ) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized,
    {
        todo!()
    }

    async fn close_connection(self: &mut Self) -> Result<(), Box<dyn std::error::Error>> {
        let closed = self
            .close_connection()
            .await
            .expect("nao foi possivel fechar conexao");
        Ok(closed)
    }
}
