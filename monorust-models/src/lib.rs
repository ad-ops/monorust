use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Debug)]
pub struct CheckoutCodeRequest {
    pub user: String,
    pub module: String,
    pub env: String,
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Checkout {
    pub id: i64,
    pub module: String,
    pub environment: String,
    pub user: String,
}
