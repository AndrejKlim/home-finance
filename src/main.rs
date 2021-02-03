mod controller;
pub mod service;
pub mod model;

use actix_web::{App, HttpServer, web};
use std::env;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    simple_logger:: init_with_level(log::Level::Info).unwrap();

    // Get the port number to listen on.
    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a number");

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
        .bind(("0.0.0.0", port))?
        .run()
        .await
}