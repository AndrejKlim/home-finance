use super::Repository;
use crate::model::{Expense, ExpenseBuilder};

pub struct ExpenseRepository{
    pub  repo: Repository
}

pub trait ExpenseRepo{
    fn get_expenses(&mut self) -> Vec<Expense>;
    fn create_expense(&mut self, expense: Expense);
    fn delete_expense(&mut self, expense: Expense);
    fn get_expense_by_id(&mut self, id: i32) -> Expense;
}

impl ExpenseRepository{
    pub fn new() -> ExpenseRepository{ ExpenseRepository{ repo: Repository::new() } }
}

impl ExpenseRepo for ExpenseRepository{
    fn get_expenses(&mut self) -> Vec<Expense> {
        let mut expenses: Vec<Expense> = vec![];
        let rows =
            match self.repo.client.query("select * from finance.expense", &[]){
                Ok(r) => r,
                Err(_) => Vec::new()
            };
        for row in rows{
            expenses.push(ExpenseBuilder::new()
                .id(row.get(0))
                .expense_date(row.get(1))
                .price(row.get(2))
                .description(row.get(3))
                .category_id(row.get(4))
                .user_id(row.get(5))
                .build())
        }
        expenses
    }

    fn create_expense(&mut self, expense: Expense) {
        self.repo.client.query(
            "insert into finance.expense(expense_date, price, description, category_id, user_id)\
             values($1, $2, $3, $4, $5)",
            &[&expense.expense_date,&expense.price, &expense.description,
                &expense.category_id, &expense.user_id]
        );
    }

    fn delete_expense(&mut self, id: i32) {
        self.repo.client.query("delete from finance.expense where id = $1", &[&id]);
    }

    fn get_expense_by_id(&mut self, id: i32) -> Expense {
        match self.repo.client.query_one("select * from finance.expense where id = $1", &[&id]){
            Ok(row) => ExpenseBuilder::new()
                .id(row.get(0))
                .expense_date(row.get(1))
                .price(row.get(2))
                .description(row.get(3))
                .category_id(row.get(4))
                .user_id(row.get(5))
                .build(),
            Err(_) => ExpenseBuilder::new().build()
        }
    }
}