use actix_web::{get, main, web, App, HttpServer, Responder};

#[get("/")]
async fn hello(data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name;
    format!("{}", app_name)
}

struct AppState {
    app_name: String,
}

#[main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                app_name: String::from("Rust Todo API"),
            }))
            .service(hello)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
