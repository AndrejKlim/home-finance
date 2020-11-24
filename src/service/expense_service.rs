use crate::repository::expense_repo::{ExpenseRepository, ExpenseRepo};
use crate::model::{Expense, ExpenseBuilder};
use chrono::{DateTime, Local};

pub struct ExpenseService{
    repo: ExpenseRepository
}

impl ExpenseService{
    pub fn new() -> ExpenseService { ExpenseService{ repo:ExpenseRepository::new() } }

    pub fn get_expenses(&mut self) -> Vec<Expense>{
        self.repo.get_expenses()
    }

    pub fn create_expense(&mut self, expense: Expense) {
        self.repo.create_expense(expense);
    }

    pub fn delete_expense(&mut self, id: i32){
        self.repo.delete_expense();
    }

    pub fn get_expense_by_id(&mut self, id: i32) -> Expense{
        self.repo.get_expense_by_id(id)
    }
}