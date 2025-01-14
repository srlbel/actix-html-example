use actix_files as fs;
use actix_web::{get, middleware, App, HttpRequest, HttpServer, Result};

#[get("/")]
async fn index(_req: HttpRequest) -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("./dist/index.html")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log::info!("Starting HTTP at http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default().log_target("http_log"))
            .service(fs::Files::new("/static", "./dist").index_file("index.html"))
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
