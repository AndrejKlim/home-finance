use super::{Repository};
use crate::model::{User};

pub struct UserRepository{
    repo: Repository
}

pub trait UserRepo{
    fn get_users(&mut self) -> Vec<User>;
    fn create_user(&mut self, user_name: String);
    fn delete_user(&mut self, id: i32);
    fn get_user_by_id(&mut self, id: i32) -> User;
}

impl UserRepository{
    pub fn new() -> UserRepository{
        UserRepository{ repo: Repository::new() }
    }
}

impl UserRepo for UserRepository{
    fn get_users(&mut self) -> Vec<User> {
        let mut users : Vec<User> = vec![];
        let rows =
            match self.repo.client.query("select * from finance.users", &[]) {
                Ok(i) => i,
                Err(_) => Vec::new() };
        for row in rows{
            users.push(User::new(row.get(0), row.get(1)))
        }
        users
    }

    fn create_user(&mut self, user_name: String) {
        self.repo.client.query("insert into finance.users(name) values ($1)", &[&user_name]);
    }

    fn delete_user(&mut self, id: i32) {
        unimplemented!()
    }

    fn get_user_by_id(&mut self, id: i32) -> User {
        unimplemented!()
    }
}