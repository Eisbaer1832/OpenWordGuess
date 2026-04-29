pub mod database;
pub mod routes;
use actix_web::{web, App, HttpServer};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server starting at http://127.0.0.1:8080");
    database::init_database();
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(routes::home))
            .route("/suggest", web::get().to(routes::suggest))
            .route("/api/categories", web::get().to(routes::get_categories))
            .route("/api/words/{language}/{category}", web::get().to(routes::get_words_by_category))
            .route("/api/words/{language}", web::get().to(routes::get_words))
            .route("/api/addWord", web::post().to(routes::add_word))

    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}