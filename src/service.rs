use postgres::{Client, NoTls, Error};
use log::info;
use crate::model::{User, Expense, Category, ExpenseBuilder, ExpenseCreateRequest};

// TODO переделать этот метод чтобы было одно подключение и все его дергали
fn get_db_client() -> Client {
    let conn_str = "postgresql://postgres:postgres@localhost/finance";
    match Client::connect(conn_str, NoTls) {
        Ok(Client) => { Client }
        Err(e) => { panic!("Error during connection to db") }
    }
}

// Users functions
pub fn get_users() -> Vec<User> {
    let mut users: Vec<User> = vec![];
    let mut client = get_db_client();
    let rows =
        match client.query("select * from finance.users", &[]) {
            Ok(i) => i,
            Err(_) => Vec::new()
        };
    for row in rows {
        users.push(User::new(row.get(0), row.get(1)))
    }
    users
}

pub fn create_user(name: &str) {
    let mut client = get_db_client();
    info!("Creating user with name = {}", &name);
    client.query("insert into finance.users(name) values ($1)", &[&name]);
}

// Categories functions
pub fn get_categories() -> Vec<Category> {
    let mut client = get_db_client();
    let mut categories: Vec<Category> = vec![];
    let rows =
        match client.query("select * from finance.category", &[]) {
            Ok(i) => i,
            Err(_) => Vec::new()
        };
    for row in rows {
        categories.push(Category::new(row.get(0), row.get(1)))
    }
    categories
}

pub fn create_category(name: &str) {
    let mut client = get_db_client();
    client.query("insert into finance.category(name) values ($1)", &[&name]);
}

pub fn get_category_by_id(id: i32) -> Category {
    let mut client = get_db_client();
    match client.query_one("select * from finance.category where id = $1", &[&id]) {
        Ok(row) => Category::new(row.get(0), row.get(1)),
        Err(_) => Category::new(0, String::new())
    }
}

// Expenses functions
pub fn get_expenses() -> Vec<Expense> {
    let mut client = get_db_client();
    let mut expenses: Vec<Expense> = vec![];
    let rows =
        match client.query("select * from finance.expense", &[]){
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

pub fn create_expense(expense: ExpenseCreateRequest) {
    let mut client = get_db_client();
    client.query(
        "insert into finance.expense(expense_date, price, description, category_id, user_id)\
             values($1, $2, $3, $4, $5)",
        &[&expense.expense_date,&expense.price, &expense.description,
            &expense.category_id, &expense.user_id]
    );
}

pub fn delete_expense(id: i32) {
    let mut client = get_db_client();
    client.query("delete from finance.expense where id = $1", &[&id]);
}

pub fn get_expense_by_id(id: i32) -> Expense {
    let mut client = get_db_client();
    match client.query_one("select * from finance.expense where id = $1", &[&id]){
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