use actix_web::{web, App, HttpServer, Responder};

async fn hello() -> impl Responder {
    "Hello depuis Render !"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(hello)))
        .bind(("0.0.0.0", std::env::var("PORT").unwrap().parse().unwrap()))?
        .run()
        .await
}
