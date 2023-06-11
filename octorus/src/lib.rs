pub mod ordatabase;
pub mod ormysql;
pub mod orresult;

pub fn get_supported_dbms() -> Vec<String> {
    vec![
        String::from("MySql"),
    ]
}
