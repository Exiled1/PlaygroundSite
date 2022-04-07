mod business;
mod reviews;
use tokio;
use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};

struct AppState{
    app_name: String,
}


#[tokio::main]
async fn main() -> std::io::Result<()> {
    let server_data = web::Data::new(AppState{
        app_name: "Actix-web".into(),
    });

    let app = move || {
        App::new()
            .app_data(server_data.clone())
            .service(get_business)
    };
    
    HttpServer::new(app)
        .bind(("127.0.0.1", 33333))?
        .run()
        .await
}


#[get("buisiness")]
async fn get_business() -> impl Responder{
    HttpResponse::Ok().body("Here's a buisiness endpoint.")
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;
    format!("Hello {app_name}!")
}