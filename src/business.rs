use std::sync::Arc;

use actix_web::{body::BoxBody, http::header::ContentType, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_json::json;
// use std::sync::Arc;

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
pub struct Photo {
    pub user_name: String,
    pub photo_id: usize,
    pub photo_url: String,
    pub photo_caption: Option<String>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct UserReviews(Vec<(String, Review)>);

/// The User reviews impl deals with working with reviews on a lower level.
impl UserReviews {
    fn new(name: String, review: Review) -> Self {
        UserReviews(vec![(name, review)])
    }

    // Adds a review to the list and checks if the user has already reviewed the business. If they have then it will return an error.
    fn add_review(&mut self, user: String, review: Review) {
        // If we find any reviews with the same user, then we will return withoutt doing anything.
        if user == "Anonymous" {
            self.0.push((user, review));
        } else if self.0.iter().any(|(user_name, _)| user_name == &user) {
            return;
        } else {
            self.0.push((user, review));
        }
    }

    // Finds a review by the user name for a specific business.
    fn get_review(&self, user_name: String) -> Option<&Review> {
        Some(
            &self
                .0
                .iter()
                .find(|(user_name_in_review, _)| user_name_in_review == &user_name)
                .unwrap()
                .1,
        )
    }

    fn delete_review(&mut self, user: String) {
        // Checks through the array and deletes anything that matches the username.
        if user == "Anonymous" {
            return;
        } else {
            self.0.retain(|(user_name, _)| user_name != &user);
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct BusinessResponse {
    pub business: Business,
    pub reviews: Option<UserReviews>,
    pub photos: Option<Vec<Photo>>,
}

impl BusinessResponse {
    pub fn new(business: Business, reviews: Option<UserReviews>, photos: Option<Vec<Photo>>) -> Self {
        BusinessResponse { business, reviews, photos}
    }

    pub fn delete_business_review(&mut self, user: String) -> HttpResponse {
        if let Some(reviews) = &mut self.reviews {
            reviews.delete_review(user.clone());
            HttpResponse::Ok().json(json!({
                "message": "Review deleted.",
                "deleted_review": user,
            }))
        } else {
            HttpResponse::Ok().json(json!({
                "message": "No reviews to delete.",
            }))
        }
    }

    pub fn add_business_review(&mut self, user: String, review: Review) -> HttpResponse {
        if let Some(reviews) = &mut self.reviews {
            reviews.add_review(user.clone(), review.clone());
            HttpResponse::Ok().json(json!({
                "message": "Review added.",
                "added_review": user,
            }))
        } else {
            HttpResponse::Ok().json(json!({
                "message": "No reviews to add.",
            }))
        }
    }

    pub fn update_business_review(&mut self, user: String, review: Review) -> HttpResponse {
        if let Some(reviews) = &mut self.reviews {
            reviews.delete_review(user.clone());
            reviews.add_review(user.clone(), review.clone());
            HttpResponse::Ok().json(json!({
                "message": "Review updated.",
                "updated_review": review,
            }))
        } else {
            HttpResponse::Ok().json(json!({
                "message": "No reviews to update.",
            }))
        }
    }

    pub fn get_business_reviews(&self) -> HttpResponse {
        if let Some(reviews) = &self.reviews {
            HttpResponse::Ok().json(reviews)
        } else {
            HttpResponse::Ok().json(json!({
                "message": "No reviews to get.",
            }))
        }
    }

    pub fn show_business_reviews(&self, user_name: String) -> HttpResponse {
        if let Some(reviews) = &self.reviews {
            let mut reviews_to_show = reviews.0.clone();
            let mut user_reviews = reviews_to_show
                .iter()
                .filter(|(name, review)| name == &user_name)
                .collect::<Vec<_>>();
            user_reviews
                .sort_by(|(_, review_1), (_, review_2)| review_1.rating.cmp(&review_2.rating));
            // let index_at = page * per_page;

            HttpResponse::Ok().json(user_reviews)
        } else {
            HttpResponse::Ok().json(json!({
                "message": "No reviews to get.",
            }))
        }
    }

    pub fn add_business_photo(&mut self, user_name: String, photo: Photo) -> HttpResponse {
        if let Some(photos) = &mut self.photos {
            photos.push(photo.clone());
            HttpResponse::Ok().json(json!({
                "message": "Photo added.",
                "added_photo": photo.clone(),
            }))
        } else {
            HttpResponse::Ok().json(json!({
                "message": "No photos to add.",
            }))
        }
    }

    pub fn delete_business_photo(&mut self, user_name: String, photo_id: usize) -> HttpResponse {
        if let Some(photos) = &mut self.photos {
            photos.retain(|photo| photo.user_name != user_name && photo.photo_id != photo_id);
            HttpResponse::Ok().json(json!({
                "message": "Photo deleted.",
                "deleted_photo": photo_id,
            }))
        } else {
            HttpResponse::Ok().json(json!({
                "message": "No photos to delete.",
            }))
        }
    }

    pub fn update_business_photo(&mut self, user_name: String, photo: Photo) -> HttpResponse {
        if let Some(photos) = &mut self.photos {
            
            photos.retain(|photo| photo.user_name != user_name);
            photos.push(photo.clone());
            HttpResponse::Ok().json(json!({
                "message": "Photo updated.",
                "updated_photo": photo,
            }))
        } else {
            HttpResponse::Ok().json(json!({
                "message": "No photos to update.",
            }))
        }
    }
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
