use chrono::{Date, DateTime, Local};

pub struct User {
    pub id : i32,
    pub name : String
}

pub struct Category{
    pub id : i32,
    pub name : String
}

pub struct Expense{
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
