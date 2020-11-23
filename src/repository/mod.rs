pub mod user_repo;
pub mod category_repo;
pub mod expense_repo;

use postgres::{Client, NoTls, Error};
use crate::model::{User};

pub struct Repository{
    pub connect_string : String,
    pub client : Client
}

impl Repository{
    pub fn new() -> Repository{
        let conn_str = "postgresql://rust_user:password@localhost/home_fin";
        Repository{
            connect_string: conn_str.to_string(),
            client: Client::connect(conn_str, NoTls).unwrap()
        }
    }
}