use crate::repository::user_repo::{UserRepository, UserRepo};
use crate::model::User;

pub struct UserService{
    repo: UserRepository
}

impl UserService{
    pub fn new() -> UserService { UserService{ repo: UserRepository::new() } }

    pub fn get_users(&mut self) -> Vec<User> {
        self.repo.get_users()
    }

    pub fn create_user(&mut self, name: String){
        self.repo.create_user(name);
    }

    pub fn delete_user(&mut self, id:i32){
        // TODO
        // self.repo.delete_user(id);
    }
}