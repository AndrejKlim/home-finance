mod controller;
pub mod service;
pub mod model;

use actix_web::{App, HttpServer, web};
use log::info;
use std::env;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    simple_logger:: init_with_level(log::Level::Info).unwrap();

    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/")
                    .service(controller::index)
                    .service(controller::get_users)
                    .service(controller::create_user)
                    .service(controller::get_expenses)
                    .service(controller::get_expense_by_id)
                    .service(controller::create_expense)
                    .service(controller::get_categories)
                    .service(controller::get_categories_by_id)
                    .service(controller::create_category)
            )
    })
        .workers(10)
        .keep_alive(15)
        .bind(get_url())?
        .run()
        .await
}

fn get_url() -> String{
    let mut addr = "127.0.0.1:".to_owned();
    let port = env::var("PORT").ok().unwrap_or("8080".to_owned());
    addr.push_str(port.as_str());
    addr
}