use serde::Deserialize;

#[derive(Deserialize)]
pub struct Review{
    pub rating: usize,
    pub dollar_signs: usize,
    pub review: Option<String>
}