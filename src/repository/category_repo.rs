use super::{Repository};
use crate::model::{Category};

pub struct CategoryRepository{
    repo: Repository
}

pub trait CategoryRepo{
    fn get_categories(&mut self) -> Vec<Category>;
    fn create_category(&mut self, category_name: String);
    fn delete_category(&mut self, id: i32);
    fn get_category_by_id(&mut self, id: i32) -> Category;
}

impl CategoryRepository{
    pub fn new() -> CategoryRepository{
        CategoryRepository{ repo: Repository::new() }
    }
}

impl CategoryRepo for CategoryRepository{
    fn get_categories(&mut self) -> Vec<Category> {
        let mut categories: Vec<Category> = vec![];
        let rows =
            match self.repo.client.query("select * from finance.category", &[]){
                Ok(i) => i,
                Err(_) => Vec::new()
            };
        for row in rows{
            categories.push(Category::new(row.get(0), row.get(1)))
        }
        categories
    }

    fn create_category(&mut self, category_name: String) {
        self.repo.client.query("insert into finance.category(name) values ($1)", &[&category_name]);
    }

    fn delete_category(&mut self, id: i32) {
        unimplemented!() // TODO
    }

    fn get_category_by_id(&mut self, id: i32) -> Category {
        match self.repo.client.query_one("select * from finance.category where id = $1", &[&id]){
            Ok(row) => Category::new(row.get(0), row.get(1)),
            Err(_) => Category::new(0,String::new())
        }
    }
}