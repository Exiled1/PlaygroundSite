use serde::Deserialize;

#[derive(Deserialize)]
pub struct Business{
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

#[derive(Deserialize)]

pub struct Category {
    pub main_category: String,
    pub subcategory: String,
}
