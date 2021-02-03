use actix_web::{Responder, HttpResponse, get, post, web};
use log::info;
use tera::{Context, Tera};
use crate::model::{ExpenseCreateRequest, ExpenseCreateForm};
use crate::service::create_expense;

const INDEX_PAGE: &str = "index.html";
const ADD_PAGE: &str = "add_expense_form.html";

#[get("")]
pub async fn index(tera: web::Data<Tera>) ->  impl Responder{
    info!("Start page");

    let mut data = Context::new();
    data.insert("title", "Домашние расходы");
    data.insert("body_header", "Домашние расходы");
    data.insert("add_href_name", "Добавить");
    let rendered = tera.render(INDEX_PAGE, &data).unwrap();

    HttpResponse::Ok().body(rendered)
}

#[get("/add")]
pub async fn add_expense_form(tera: web::Data<Tera>) -> impl Responder{
    info!("Add expense page");

    let mut data = Context::new();
    let rendered = tera.render(ADD_PAGE, &data).unwrap();

    HttpResponse::Ok().body(rendered)
}

#[post("/add")]
pub async fn handle_add_expense(form: web::Form<ExpenseCreateRequest>) -> impl Responder{
    create_expense(form.0);
    HttpResponse::Created() //TODO add redirect to add page
}