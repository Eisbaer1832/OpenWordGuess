use crate::database;
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

pub(crate) async fn home() -> impl Responder {
    HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::html())
        .body(include_str!("../static/README.html"))
}


pub(crate) async fn suggest() -> impl Responder {
    HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::html())
        .body(include_str!("../static/suggest.html"))
}



pub(crate) async fn get_categories() -> impl Responder {
    let words = database::get_categories();
    let response = words;
    web::Json(response)
}

pub async fn get_words(
    path: web::Path<String>
) -> impl Responder {
    let language = path.into_inner();
    let words = database::get_words(&language);
    let response = words;
    web::Json(response)
}


pub async fn get_words_by_category(
    path: web::Path<(String, String)>
) -> impl Responder {
    let (language, category) = path.into_inner();
    let words = database::get_words_by_category(&category, &language);
    let response = words;
    web::Json(response)
}


#[derive(Deserialize)]
pub struct AddWordRequest {
    pub word: String,
    pub language: String,
    pub category: String,
    pub taboos: Vec<String>,
}
pub async fn add_word(payload: web::Json<AddWordRequest>) -> impl Responder {
    database::suggest_word(payload);
    HttpResponse::Ok()
}