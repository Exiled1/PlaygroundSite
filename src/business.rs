use std::fs::File;

use actix_web::{body::BoxBody, http::header::ContentType, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_json::{error, from_reader, from_slice, json, Value as JSON};
#[derive(Deserialize, Serialize, Clone)]
pub struct Business {
    pub name: String,
    pub street_addr: String,
    pub city: String,
    pub state: String,
    pub zip: usize,
    pub phone_num: usize, // TODO: Make a struct for phone numbers if it's important.
    pub category: Category,
    pub email: Option<String>,
    pub website: Option<String>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Category {
    pub main_category: String,
    pub subcategory: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Review {
    pub rating: usize,
    pub dollar_signs: usize,
    pub review: Option<String>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct BusinessResponse {
    pub business: Business,
    pub reviews: Option<Vec<Review>>,
}

impl Responder for BusinessResponse {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

