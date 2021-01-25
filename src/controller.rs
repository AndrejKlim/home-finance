use actix_web::{get, post, web, Responder, HttpResponse};
use crate::service;
use crate::model::{UserCreateRequest, ExpenseCreateRequest, CategoryCreateRequest};

#[get("")]
pub async fn index() ->  impl Responder{
    HttpResponse::Ok().body("Hello")
}

// Users endpoints
#[get("/users")]
pub async fn get_users() -> impl Responder{
    let users = service::get_users();
    HttpResponse::Ok().json(users)
}

#[post("/users/create")]
pub async fn create_user(user_create_request: web::Json<UserCreateRequest> ) -> impl Responder{
    // TODO добавить реализацию отлова ошибок и бросания соответствующего кода ответа
    service::create_user(&user_create_request.name);
    HttpResponse::Created()
}

// Expenses endpoints
#[get("/expenses")]
pub async fn get_expenses() -> impl Responder{
    let expenses = service::get_expenses();
    HttpResponse::Ok().json(expenses)
}

#[get("/expenses/{id}")]
pub async fn get_expense_by_id(params: web::Path<i32>) -> impl Responder{
    let expense = service::get_expense_by_id(params.0);
    HttpResponse::Ok().json(expense)
}

#[post("expenses/create")]
pub async fn create_expense(expense_create_request: web::Json<ExpenseCreateRequest>) -> impl Responder{
    // TODO добавить реализацию отлова ошибок и бросания соответствующего кода ответа
    service::create_expense(expense_create_request.0);
    HttpResponse::Created()
}

// Category endpoints
#[get("/categories")]
pub async fn get_categories() -> impl Responder{
    let categories = service::get_categories();
    HttpResponse::Ok().json(categories)
}

#[get("/categories/{id}")]
pub async fn get_categories_by_id(params: web::Path<i32>) -> impl Responder{
    let category = service::get_category_by_id(params.0);
    HttpResponse::Ok().json(category)
}

#[post("/categories/create")]
pub async fn create_category(category_create_request: web::Json<CategoryCreateRequest>) -> impl Responder{
    // TODO добавить реализацию отлова ошибок и бросания соответствующего кода ответа
    service::create_category(&category_create_request.name);
    HttpResponse::Created()
}