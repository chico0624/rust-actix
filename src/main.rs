use actix_web::{get, App, HttpResponse, HttpServer, ResponseError};
use askama::Template;
use thiserror::Error;

#[derive(Error, Debug)]
enum MyError {
    #[error("Failed to render HTML")]
    AskamaError(#[from] askama::Error),
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
async fn index() -> Result<HttpResponse, MyError> {
    let mut entries = Vec::new();
    entries.push(TodoEntry {
        id: 1,
        text: "first".to_string(),
    });
    entries.push(TodoEntry {
        id: 2,
        text: "second".to_string(),
    });

    let html = IndexTemplate { entries };

    let respons_body = html.render()?;
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(respons_body))
}

#[actix_web::main]
async fn main() -> Result<(), actix_web::Error> {
    HttpServer::new(move || App::new().service(index))
        .bind("0.0.0.0:8080")?
        .run()
        .await?;

    Ok(())
}
