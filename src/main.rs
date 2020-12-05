use actix_web::{get, App, HttpResponse, HttpServer, Responder, middleware::Logger};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/hello")]
async fn hello1() -> impl Responder {
    HttpResponse::Ok().body("Hello1 world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(hello)
            .service(hello1)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
