use crate::repository::category_repo::{CategoryRepository, CategoryRepo};
use crate::model::Category;
use std::ptr::null;

pub struct CategoryService{
    repo: CategoryRepository
}

impl CategoryService{
    pub fn new() -> CategoryService { CategoryService{ repo:CategoryRepository::new() } }

    pub fn get_categories(&mut self) -> Vec<Category>{
        self.repo.get_categories()
    }

    pub fn create_category(&mut self,  name: String) {
        self.repo.create_category(name);
    }

    pub fn delete_category(&mut self, id: i32) {
        // TODO
        // self.repo.delete_category(id);
    }
}

pub trait TCategoryService{

}
