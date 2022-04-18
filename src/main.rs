#![allow(non_snake_case)]
mod business;
mod reviews;
use tokio;
use actix_web::{get, web, App, HttpServer, Responder, HttpResponse, dev::Server};

struct AppState{
    app_name: String,
}


#[tokio::main]
async fn main() -> std::io::Result<()> {    
    create_server()?.await?;
    Ok(())
}

fn create_server()-> std::io::Result<Server>{
    let server_data = web::Data::new(AppState{
        app_name: "Belp".into(),
    });

    let app = move || {
        App::new()
            .app_data(server_data.clone())
            .service(get_business)
            .service(index)
    };
    
    Ok(HttpServer::new(app)
        .bind(("127.0.0.1", 33333))?
        .run())
}


#[get("business")]
async fn get_business() -> impl Responder{
    HttpResponse::Ok().body("Here's a buisiness endpoint.")
}

#[get("business/{id}")]
async fn set_business(_id: web::Path<String>) -> impl Responder{
    HttpResponse::Ok().body("Here's a buisiness endpoint.")
}



#[get("/")]
async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;
    format!("Hello {app_name}!")
}