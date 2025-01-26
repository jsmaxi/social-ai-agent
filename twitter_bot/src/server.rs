/*
  Actix Web Server.
  Call function init to spin up.
  Expose and visit local endpoints:
  http://localhost:8080/
  http://localhost:8080/Joe
*/

use actix_web::{web, App, HttpRequest, HttpServer, Responder};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello, {}!", name)
}

pub async fn init() -> Result<(), Box<dyn std::error::Error>> {
    println!("Actix Web Server started listening.");

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    .bind(("127.0.0.1", 8080))? // Bind to localhost on port 8080
    .run()
    .await?;

    println!("Actix Web Server stopped listening.");

    Ok(())
}
