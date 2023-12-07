use ascii_table::AsciiTable;
use std::env;
use std::io;
use std::io::Write;

use octorus::{ordatabase::ORDatabase, ormysql::ORMySql};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let mut host = String::new();
    let mut user = String::new();
    let mut password = String::new();
    let mut database = String::new();

    for (index, ele) in args.iter().enumerate() {
        if ele == "-h" {
            host.push_str(&args[index + 1]);
        }
        if ele == "-u" {
            user.push_str(&args[index + 1]);
        }
        if ele == "-p" {
            password.push_str(&args[index + 1]);
        }
        if ele == "-d" {
            database.push_str(&args[index + 1]);
        }
    }
    println!("Conectando no banco");
    let mut mysql = ORMySql::new(&host, &user, &password, &database)
        .await
        .expect("nao foi possivel conectar");
    loop {
        print!("> ");
        let mut buffer = String::new();
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut buffer)
            .expect("nao foi possivel ler stdin");
        if buffer.trim() != "exit".trim() {
            let result = mysql
                .send_query(buffer.as_str())
                .await
                .expect("nao foi possivel fazer a query");

            let ascii_table = AsciiTable::default();
            ascii_table.print(result.result_set);
        } else {
            break;
        }
    }
    println!("fechando");
}
