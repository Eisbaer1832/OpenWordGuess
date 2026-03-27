use std::string::String;
use rusqlite::Connection;

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
                taboo TEXT NOT NULL PRIMARY KEY,
                word TEXT NOT NULL,
                FOREIGN KEY (word) REFERENCES words(word)
            )",
        []
    ).unwrap();
}

fn suggest_word(word: &str, taboos: &[&str], category: &str, language: &str) {
    let conn = db_connect();

    conn.execute(
        "INSERT INTO words (word, category, language) VALUES (?1, ?2, ?3)",
        &[word, category, language]
    ).unwrap();

    for taboo in taboos {
        conn.execute(
            "INSERT INTO taboos (taboo, word) VALUES (?1, ?2)",
            &[word, taboo]
        ).unwrap();
    }
}

pub(crate) fn get_words(category: &str, language: &str) -> Vec<String> {
    let conn = db_connect();

    let mut stmt = conn.prepare(
        "SELECT word
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