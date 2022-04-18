#![allow(non_snake_case)]
mod business;
// mod reviews;
mod endpoints;
use std::collections::HashMap;
use std::sync::Arc;

use actix_web::http::header::ContentType;
use business::{Business, BusinessResponse, Review};
use serde_json::json;
// use reviews::Review;
use actix_web::{
    delete, dev::Server, get, post, put, web, App, HttpResponse, HttpServer, Responder,
};
use tokio;
use tokio::sync::RwLock;

// use crate::endpoints::AppError;

type AtomicDB = Arc<RwLock<HashMap<String, BusinessResponse>>>;

struct AppState {
    app_name: String,
    mock_database: AtomicDB,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    create_server()?.await?;

    Ok(())
}

fn create_server() -> std::io::Result<Server> {
    let server_data = web::Data::new(AppState {
        app_name: "Belp".into(),
        mock_database: Arc::new(RwLock::new(HashMap::new())),
    });
    // Shared data setup ^^^

    let app = move || {
        App::new()
            .app_data(server_data.clone()) // App data uses Arc, so I don't have to.
            .service(index)
            .service(add_business)
            .service(get_businesses)
            .service(delete_business)
            .service(set_business)
    };
    // App setup ^^^

    Ok(HttpServer::new(app).bind(("127.0.0.1", 33333))?.run())
    // Server setup ^^^
}

#[post("/business")]
async fn add_business(
    business_data: web::Json<BusinessResponse>,
    resources: web::Data<AppState>,
) -> std::io::Result<impl Responder> {
    let database = resources.mock_database.clone();
    if database
        .read()
        .await
        .contains_key(&business_data.business.name)
    {
        return Ok(HttpResponse::Conflict().body("Business already exists"));
    } else {
        database
            .write()
            .await
            .insert(business_data.business.name.clone(), business_data.clone());

        Ok(HttpResponse::Ok().json(json!({
            "return_code": 200,
            "body": {
                "payload": business_data
            }
        })))
    }
}

#[get("/business")]
async fn get_businesses(resources: web::Data<AppState>) -> std::io::Result<impl Responder> {
    let database = resources.mock_database.clone();
    let database_read: Vec<BusinessResponse> = database.read().await.values().cloned().collect();
    Ok(web::Json(database_read))
}

#[delete("/business/{business_name}")]
async fn delete_business(
    resources: web::Data<AppState>,
    business_name: web::Path<String>,
) -> std::io::Result<impl Responder> {
    let database = resources.mock_database.clone();
    let removed_business = database.write().await.remove(&business_name.to_string());
    match removed_business {
        Some(business) => Ok(HttpResponse::Ok().json(business)),
        None => Ok(HttpResponse::NotFound().json(json!({
            "notes": "Reached the deletion endpoint",
            "error": "Business not found"
        }))),
    }
}

#[get("/business/{business_name}")]
async fn set_business(
    business_name: web::Path<String>,
    resources: web::Data<AppState>,
) -> std::io::Result<impl Responder> {
    let database = resources.mock_database.clone();
    let searched_business = database
        .read()
        .await
        .get(&business_name.to_string())
        .cloned();
    match searched_business {
        Some(business) => Ok(HttpResponse::Ok().json(business)),
        None => Ok(HttpResponse::NotFound().json(json!({
            "notes": "Reached the business info specification endpoint",
            "error": "Business not found"
        }))),
    }
}

#[put("/business/{business_name}")]
async fn update_business(
    business_name: web::Path<String>,
    business_data: web::Json<BusinessResponse>,
    resources: web::Data<AppState>,
) -> std::io::Result<impl Responder> {
    let database = resources.mock_database.clone();
    let updated_business = database
        .write()
        .await
        .insert(business_name.to_string(), business_data.clone());
    match updated_business {
        Some(business) => Ok(HttpResponse::Ok().json(business)),
        None => Ok(HttpResponse::NotFound().json(json!({
            "notes": "Reached the business update endpoint",
            "error": "Business not found"
        }))),
    }
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;
    format!("Hello! Welcome to {app_name}!")
}
