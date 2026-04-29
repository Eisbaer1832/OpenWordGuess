use crate::routes::AddWordRequest;
use actix_web::web::Json;
use rusqlite::Connection;
use serde::Serialize;
use std::string::String;

fn db_connect() -> Connection {
    Connection::open("database.db").unwrap()
}

pub(crate) fn init_database() {
    let conn = db_connect();
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS words (
                word TEXT NOT NULL PRIMARY KEY,
                category TEXT NOT NULL,
                language TEXT NOT NULL,
                reviewed INTEGER NOT NULL DEFAULT 0,
                FOREIGN KEY (language) REFERENCES languages(key)
            )",
        []
    ).unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS languages (
                key TEXT NOT NULL PRIMARY KEY,
                display TEXT NOT NULL
            )",
        []
    ).unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS taboos (
                taboo TEXT NOT NULL,
                word TEXT NOT NULL,
                FOREIGN KEY (word) REFERENCES words(word),
                PRIMARY KEY (word, taboo)
            )",
        []
    ).unwrap();
}

pub(crate) fn suggest_word(r: Json<AddWordRequest>) {
    let conn = db_connect();

    conn.execute(
        "INSERT OR IGNORE INTO words (word, category, language) VALUES (?1, ?2, ?3)",
        [r.word.clone(), r.category.clone(), r.language.clone()]
    ).unwrap();

    for taboo in &r.taboos {

        conn.execute(
            "INSERT OR IGNORE INTO taboos (taboo, word) VALUES (?1, ?2)",
            [String::from(taboo), r.word.clone()]
        ).unwrap();
    }
}

pub(crate) fn get_words_by_category(category: &str, language: &str) -> Vec<String> {
    let conn = db_connect();

    let mut stmt = conn.prepare(
        "SELECT *
        FROM words AS w
        WHERE w.language = ?1
        AND w.category = ?2
        AND w.reviewed = 1"
    ).unwrap();
    let words_iter = stmt.query_map([language, category], |row| {
        row.get::<_, String>(0)
    }).unwrap();

    let words: Vec<String> = words_iter
        .map(|res| res.unwrap())
        .collect();
    words
}

#[derive(Serialize)]
pub struct WordEntry {
    pub word: String,
    pub taboos: Vec<String>,
    pub category: String,
}

pub(crate) fn get_words(language: &str) -> Vec<WordEntry> {
    let conn = db_connect();

    let mut stmt = conn.prepare(
        "SELECT word, category
         FROM words
         WHERE language = ?1
         AND reviewed = 1"
    ).unwrap();

    let word_entries = stmt.query_map([language], |row| {
        let word: String = row.get(0)?;
        let category: String = row.get(1)?;

        let mut taboo_stmt = conn.prepare(
            "SELECT taboo
             FROM taboos
             WHERE word = ?1"
        ).unwrap();

        let taboo_iter = taboo_stmt.query_map([&word], |t_row| {
            t_row.get::<_, String>(0)
        }).unwrap();

        let taboos: Vec<String> = taboo_iter
            .map(|res| res.unwrap())
            .collect();

        Ok(WordEntry { word, category, taboos })
    }).unwrap();

    word_entries.map(|res| res.unwrap()).collect()
}

pub(crate) fn get_categories() -> Vec<String> {
    let conn = db_connect();
    let mut stmt = conn.prepare("SELECT DISTINCT category FROM words").unwrap();
    let category_iter = stmt.query_map([], |row| {
        row.get::<_, String>(0)
    }).unwrap();
    let categories: Vec<String> = category_iter
        .map(|res| res.unwrap())
        .collect();
    categories

}