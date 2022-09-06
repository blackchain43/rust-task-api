mod api;
use api::task::{
    get_task
};
use actix_web::{HttpServer, App, web::Data, middleware::Logger};
#[actix_web::main]
fn main() {
    println!("Hello, world!");
}
