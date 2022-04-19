#![allow(non_snake_case)]
mod business;
use std::collections::HashMap;
use std::sync::Arc;

use business::BusinessResponse;
use serde_json::json;
// use reviews::Review;
use actix_web::{
    delete, dev::Server, get, post, put, web, App, HttpResponse, HttpServer, Responder,
};
use tokio;
use tokio::sync::RwLock;

use crate::business::{Review, Photo};

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
            .service(find_business)
            .service(web::scope("/review")
                .service(add_review)
                .service(delete_review)
                .service(update_review))
            .service(web::scope("/photos")
                .service(add_photo)
                .service(delete_photo)
                .service(update_photo))

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
async fn find_business(
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
    // Might be a good idea to make this read to check if it exists instead of potentially taking up a writing spot in the queue.
    let updated_business = database
        .write()
        .await
        .insert(business_name.to_string(), business_data.clone());
    
    match updated_business {
        Some(business) => Ok(HttpResponse::Ok().json(json!({
            "success": true,
            "body": "Replaced an old business!",
            "previous_business": business,
            "new_business": business_data
        }))),
        None => Ok(HttpResponse::Ok().json(json!({ // Returns None if the key didn't exist. It still added a new one.
            "success": true,
            "body": "Created a new business!",
            "created_business": business_data
        })))
    }
}
// --- Reviews below ---

/// Add a new review to a business. If the content is the exact same, make two seperate reviews.
#[post("/{reviewer_name}/{business_name}")]
async fn add_review(
    params: web::Path<(String, String)>,
    review_data: web::Json<Review>,
    resources: web::Data<AppState>,
) -> std::io::Result<impl Responder> {
    let (mut reviewer_name, business_name) = params.into_inner();
    // Yeah, this is unintended behavior. Since I should always assume that a user sends their name in the request.
    // Currently if a user name is Anonymous, it gets treated differently on purpose. Anon reviews are not allowed to be deleted or edited.
    if reviewer_name.is_empty() {
        reviewer_name = "Anonymous".to_string();
    }
    let database = resources.mock_database.clone();
    let mut database_read = database.write().await;
    let business_data = database_read.get_mut(&business_name);
    // Check to see if the business exists, if not then return an error.
    if let Some(business) = business_data {
        // If the business exists, then retrieve the review list (if it exists).
        // If it does then add the sent review into the list, if it doesn't, then create the list.
        // I could probably make it so that the review list always exists, either way works.
        return Ok(business.add_business_review(reviewer_name, review_data.clone()));
    } else {
        return Ok(HttpResponse::NotFound().json(json!({
            "notes": "Reached the review addition endpoint",
            "error": "Business not found"
        })));
    }
}

#[delete("/{reviewer_name}/{business_name}")]
async fn delete_review(
    params: web::Path<(String, String)>,
    resources: web::Data<AppState>,
) -> std::io::Result<impl Responder> {
    let (reviewer_name, business_name) = params.into_inner();
    let database = resources.mock_database.clone();
    let mut database_read = database.write().await;
    let business_data = database_read.get_mut(&business_name);
    // Check to see if the business exists, if not then return an error.
    if let Some(business) = business_data {
        // If the business exists, then retrieve the review list (if it exists).
        // If it does then add the sent review into the list, if it doesn't, then create the list.
        // I could probably make it so that the review list always exists, either way works.
        return Ok(business.delete_business_review(reviewer_name.clone()));
    }else {
        return Ok(HttpResponse::Ok().body(format!("Deleted {reviewer_name}'s review from {business_name}")));
    }
}

#[put("/{reviewer_name}/{business_name}")]
async fn update_review(
    params: web::Path<(String, String)>,
    review_data: web::Json<Review>,
    resources: web::Data<AppState>,
) -> std::io::Result<impl Responder> {
    let (reviewer_name, business_name) = params.into_inner();
    let database = resources.mock_database.clone();
    let mut database_read = database.write().await;
    let business_data = database_read.get_mut(&business_name);
    // Check to see if the business exists, if not then return an error.
    if let Some(business) = business_data {
        // If the business exists, then retrieve the review list (if it exists).
        // If it does then update the sent review into the list, if it doesn't, then exit.
        return Ok(business.update_business_review(reviewer_name.clone(), review_data.into_inner()));
    }else {
        return Ok(HttpResponse::Ok().body(format!("Updated {reviewer_name}'s review from {business_name}")));
    }
}

#[get("/{reviewer_name}/{business_name}")]
async fn show_paginated_reviews(
    params: web::Path<(String, String)>,
    resources: web::Data<AppState>,
) -> std::io::Result<impl Responder> {
    let (reviewer_name, business_name) = params.into_inner();
    let database = resources.mock_database.clone();
    let database_read = database.read().await;
    let business_data = database_read.get(&business_name);
    // Check to see if the business exists, if not then return an error.
    if let Some(business) = business_data {
        // If the business exists, then retrieve the review list (if it exists).
        // If it does then add the sent review into the list, if it doesn't, then create the list.
        // I could probably make it so that the review list always exists, either way works.
        return Ok(business.show_business_reviews(reviewer_name.clone()));
    }else {
        return Ok(HttpResponse::Ok().body(format!("Showing {reviewer_name}'s reviews from {business_name}")));
    }
}
// --- Photos API below ---

#[post("/photos/{user_name}/{business_name}")]
async fn add_photo(
    params: web::Path<(String, String)>,
    photo_data: web::Json<Photo>,
    resources: web::Data<AppState>,
) -> std::io::Result<impl Responder> {
    let (user_name, business_name) = params.into_inner();
    let database = resources.mock_database.clone();
    let mut database_read = database.write().await;
    let business_data = database_read.get_mut(&business_name);
    // Check to see if the business exists, if not then return an error.
    if let Some(business) = business_data {
        // If the business exists, then retrieve the review list (if it exists).
        // If it does then add the sent review into the list, if it doesn't, then create the list.
        // I could probably make it so that the review list always exists, either way works.
        return Ok(business.add_business_photo(user_name.clone(), photo_data.clone()));
    }else {
        return Ok(HttpResponse::Ok().body(format!("Added a photo to {business_name}")));
    }
}

#[delete("/s{user_name}/{business_name}/{photo_id}")]
async fn delete_photo(
    params: web::Path<(String, String, usize)>,
    resources: web::Data<AppState>,
) -> std::io::Result<impl Responder> {
    let (user_name, business_name, photo_id) = params.into_inner();
    let database = resources.mock_database.clone();
    let mut database_read = database.write().await;
    let business_data = database_read.get_mut(&business_name);
    // Check to see if the business exists, if not then return an error.
    if let Some(business) = business_data {
        // If the business exists, then retrieve the review list (if it exists).
        // If it does then add the sent review into the list, if it doesn't, then create the list.
        // I could probably make it so that the review list always exists, either way works.
        return Ok(business.delete_business_photo(user_name.clone(), photo_id.clone()));
    }else {
        return Ok(HttpResponse::Ok().body(format!("Deleted a photo from {business_name}")));
    }
}

#[put("/{user_name}/{business_name}")]
async fn update_photo(
    params: web::Path<(String, String)>,
    photo_data: web::Json<Photo>,
    resources: web::Data<AppState>,
) -> std::io::Result<impl Responder> {
    let (user_name, business_name) = params.into_inner();
    let database = resources.mock_database.clone();
    let mut database_read = database.write().await;
    let business_data = database_read.get_mut(&business_name);
    // Check to see if the business exists, if not then return an error.
    if let Some(business) = business_data {
        // If the business exists, then retrieve the review list (if it exists).
        // If it does then update the sent review into the list, if it doesn't, then exit.
        return Ok(business.update_business_photo(user_name.clone(), photo_data.clone()));
    }else {
        return Ok(HttpResponse::Ok().body(format!("Updated a photo from {business_name}")));
    }
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;
    format!("Hello! Welcome to {app_name}!")
}
