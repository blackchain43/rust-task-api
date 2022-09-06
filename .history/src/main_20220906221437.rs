mod api;
use api::task::{
    get_task
};
use actix_web::{HttpServer, App, web::Data, middleware::Logger};
#[actix_web::main]
fn main() -> std::io::Result<()> {
   std::env::set_var("RUST_LOG", "actix_web=info");
}
