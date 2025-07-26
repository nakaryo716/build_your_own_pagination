use crate::models::article::Article;

pub struct SearchResponse {
    pub articles: Vec<Article>,
    pub page: u32,
    pub per_page: u32,
    pub total: u32,
    pub total_page: u32,
}
