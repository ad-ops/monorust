use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CheckoutCodeRequest {
    pub user: String,
    pub module: String,
    pub env: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Checkout {
    pub id: i64,
    pub module: String,
    pub environment: String,
    pub user: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Builds {
    pub id: i64,
    pub module: String,
}