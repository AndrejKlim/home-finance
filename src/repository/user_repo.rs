use super::{Repository};
use crate::model::{User};

pub struct UserRepository{
    repo: Repository
}

pub trait UserRepo{
    fn get_users(&mut self) -> Vec<User>;
    fn create_user(&mut self, user: User);
    fn delete_user(&mut self, user: User);
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

    fn create_user(&mut self, user: User) {
        unimplemented!()
    }

    fn delete_user(&mut self, user: User) {
        unimplemented!()
    }

    fn get_user_by_id(&mut self, id: i32) -> User {
        unimplemented!()
    }
}