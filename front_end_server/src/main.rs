use actix_files::Files;
use actix_web::middleware;
use actix_web::App;
use actix_web::HttpServer;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(Files::new("/", "./dist/").index_file("index.html"))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
