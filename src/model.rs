use chrono::{DateTime, Local};
use serde::{Serialize,Deserialize};

#[derive(Serialize)]
pub struct User {
    pub id : i32,
    pub name : String
}

#[derive(Deserialize)]
pub struct UserCreateRequest{
    pub name: String
}

#[derive(Serialize)]
pub struct Category{
    pub id : i32,
    pub name : String
}

#[derive(Deserialize)]
pub struct CategoryCreateRequest{
    pub name : String
}

#[derive(Serialize)]
pub struct Expense{
    pub id : i32,
    pub expense_date : DateTime<Local>,
    pub price : f32,
    pub description : String,
    pub category_id : i32,
    pub user_id : i32
}

#[derive(Deserialize)]
pub struct ExpenseCreateRequest{
    pub expense_date : DateTime<Local>,
    pub price : f32,
    pub description : String,
    pub category_id : i32,
    pub user_id : i32
}

pub struct ExpenseBuilder{
    pub id : i32,
    pub expense_date : DateTime<Local>,
    pub price : f32,
    pub description : String,
    pub category_id : i32,
    pub user_id : i32
}

pub struct PlannedExpense{
    pub id : i32,
    pub name : String,
    pub priority : i32,
    pub category_id : i32,
    pub user_id : i32
}

impl User{
    pub fn new(id: i32, name: String) -> User{
        User{id, name}
    }
}

impl Category{
    pub fn new(id: i32, name: String) -> Category{
        Category{id, name}
    }
}

impl ExpenseBuilder{
    pub fn new() -> ExpenseBuilder{
        ExpenseBuilder{
            id:0,
            expense_date: Local::now(),
            price: 0.0,
            description: String::new(),
            category_id: 1,
            user_id: 1
        }
    }

    pub fn id(&mut self, id: i32) -> &mut ExpenseBuilder{
        self.id = id;
        self
    }

    pub fn expense_date(&mut self, expense_date: DateTime<Local>) -> &mut ExpenseBuilder{
        self.expense_date = expense_date;
        self
    }

    pub fn price(&mut self, price: f32) -> &mut ExpenseBuilder{
        self.price = price;
        self
    }

    pub fn description(&mut self, description: String) -> &mut  ExpenseBuilder{
        self.description = description;
        self
    }

    pub fn category_id(&mut self, category_id: i32) -> &mut ExpenseBuilder{
        self.category_id = category_id;
        self
    }

    pub fn user_id(&mut self, user_id: i32) -> &mut ExpenseBuilder{
        self.user_id = user_id;
        self
    }

    pub fn build(&self) -> Expense{
        Expense {
            id: self.id,
            expense_date: self.expense_date,
            price: self.price,
            description: self.description.clone(),
            category_id: self.category_id,
            user_id: self.user_id
        }
    }
}
