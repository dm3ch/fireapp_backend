#[macro_use]
extern crate diesel;

use actix_web::{get, post, web, App, Error, HttpResponse, HttpServer, Responder, middleware::Logger};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod report;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[post("/api/report")]
async fn add_report(    
    pool: web::Data<DbPool>,
    form: web::Json<report::InsertableReport>,
) -> Result<HttpResponse, Error> {
    // let conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    // let user = web::block(move || report::insert_report(&form.name, &conn))
    //     .await
    //     .map_err(|e| {
    //         eprintln!("{}", e);
    //         HttpResponse::InternalServerError().finish()
    //     })?;
    
    println!("{:?}", form);
    Ok(HttpResponse::Ok().body("Hello world!"))
}

#[get("/api/report")]
async fn get_report() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    dotenv::dotenv().ok();

    // set up database connection pool
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let bind_addr = std::env::var("LISTEN_ADDR").expect("LISTEN_ADDR");
    println!("Starting server at: {}", &bind_addr);

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(Logger::default())
            .service(add_report)
            .service(get_report)
    })
    .bind(&bind_addr)?
    .run()
    .await
}
