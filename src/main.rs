use actix_web::{get, web, App, HttpResponse, HttpServer, ResponseError};
use askama::Template;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;
use thiserror::Error;

#[derive(Error, Debug)]
enum MyError {
    #[error("Failed to render HTML")]
    AskamaError(#[from] askama::Error),
    #[error("Failed to get connection")]
    ConnectionPoolError(#[from] r2d2::Error),
    #[error("Failed SQL execution")]
    SQLiteError(#[from] rusqlite::Error),
}
impl ResponseError for MyError {}

struct TodoEntry {
    id: u32,
    text: String,
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    entries: Vec<TodoEntry>,
}

#[get("/")]
async fn index(db: web::Data<Pool<SqliteConnectionManager>>) -> Result<HttpResponse, MyError> {
    let conn = db.get()?;

    let sql = "SELECT id, text FROM todo";
    let mut statement = conn.prepare(sql)?;

    let rows = statement.query_map(params![], |row| {
        let id = row.get(0)?;
        let text = row.get(1)?;

        Ok(TodoEntry { id, text })
    })?;

    let mut entries = Vec::new();

    for row in rows {
        entries.push(row?);
    }

    let html = IndexTemplate { entries };

    let respons_body = html.render()?;
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(respons_body))
}

#[actix_web::main]
async fn main() -> Result<(), actix_web::Error> {
    let manager = SqliteConnectionManager::file("todo.db");
    let pool = Pool::new(manager).expect("Failed to initialize the connection pool.");

    let conn = pool.get().expect("Failed to the connection from the pool.");

    // テーブルがない場合、作成
    let sql = "CREATE TABLE IF NOT EXISTS todo(
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        text TEXT NOT NULL
    )";

    let params = params![];
    conn.execute(sql, params)
        .expect("Failed to create a table `todo`.");

    HttpServer::new(move || App::new().service(index).data(pool.clone()))
        .bind("0.0.0.0:8080")?
        .run()
        .await?;

    Ok(())
}
