use actix_web::{web, HttpResponse, Responder};
use crate::{database};

pub(crate) async fn home() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the OpenWordGuess Server!")
}

pub async fn get_words(
    path: web::Path<(String, String)>
) -> impl Responder {
    let (language, category) = path.into_inner();
    let words = database::get_words(&category, &language);
    let response = words;
    web::Json(response)
}
