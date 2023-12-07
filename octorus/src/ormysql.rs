use async_trait::async_trait;
use std::collections::BTreeMap;

use crate::{ordatabase::ORDatabase, orresult::ORResult};
use mysql::{self, prelude::Queryable, Pool};

#[derive(Debug)]
pub struct ORMySql {
    connection: mysql::PooledConn,
}

impl ORMySql {
    fn new(pool: mysql::Pool) -> Self {
        let con = pool.get_conn().expect("nao conectou");
        Self { connection: con }
    }
    pub async fn send_query(
        &mut self,
        query: &str,
        //opts: Option<T>,
    ) -> Result<ORResult, Box<dyn std::error::Error>> {
        let rows = self
            .connection
            .query_iter(query)
            .expect("nao pode fazer query");
        let mut cols_name: Vec<String> = Vec::new();
        let col_len = rows.columns().as_ref().len();
        let mut result: Vec<Vec<String>> = Vec::new();

        for i in 0..col_len {
            let name = rows
                .columns()
                .as_ref()
                .get(i)
                .unwrap()
                .name_str()
                .to_string();
            cols_name.push(name);
        }

        rows.for_each(|row| {
            let mut row_vec: Vec<String> = Vec::new();
            for i in 0..col_len {
                let v = row.as_ref().unwrap();
                let value = v.as_ref(i).unwrap().as_sql(false);
                row_vec.push(value);
            }
            result.push(row_vec);
        });

        let orresult = ORResult::new(BTreeMap::new(), cols_name, result);
        Ok(orresult)
    }
}

#[async_trait]
impl ORDatabase for ORMySql {
    async fn new(
        host: &str,
        user: &str,
        password: &str,
        database: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let connection_string = format!(
            "mysql://{0}:{1}@{2}:3306/{3}",
            user, password, host, database
        );
        let pool = Pool::new(connection_string.as_str()).expect("Nao pode conectar ao banco");
        let ormysql = ORMySql::new(pool);
        Ok(ormysql)
    }

    async fn new_with_connection_string(
        connection_string: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let pool = Pool::new(connection_string).expect("URI invalida");
        let ormysql = ORMySql::new(pool);
        Ok(ormysql)
    }

    async fn close_connection(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
}
