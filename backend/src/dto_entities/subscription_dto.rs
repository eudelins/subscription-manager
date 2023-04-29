use serde::Serialize;

#[derive(Serialize)]
pub struct SimpleSubscriptionDTO {
    pub name: String,
    pub price: f32,
    pub status: bool,
}