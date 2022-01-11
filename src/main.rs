use actix_web::{get, App, HttpResponse, HttpServer};

#[get("/")]
async fn index() -> Result<HttpResponse, actix_web::Error> {
    let respons_body = "hello world";
    Ok(HttpResponse::Ok().body(respons_body))
}

#[actix_web::main]
async fn main() -> Result<(), actix_web::Error> {
    HttpServer::new(move || App::new().service(index))
        .bind("0.0.0.0:8080")?
        .run()
        .await?;

    Ok(())
}