use postgres::{Client, NoTls};
use crate::model;


pub fn get_users() {
    let mut client =
        Client::connect("postgresql://rust_user:password@localhost/home_fin", NoTls);

    for row in client.unwrap().query("select * from users", &[]).unwrap()    {
        let user = model::User {
            id: row.get(0),
            name: row.get(1)
        };
        println!("User name = {}", user.name);
    }

}