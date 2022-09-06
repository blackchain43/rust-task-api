mod api;
use api::task::{
    get_task
};
use actix_web::{HttpServer, App, web::Data, middleware::Logger};
#[actix_web::main]
async fn main() -> std::io::Result<()> {
   std::env::set_var("RUST_LOG", "debug");
   std::env::set_var("RUST_BACKTRACE", "1");
}
