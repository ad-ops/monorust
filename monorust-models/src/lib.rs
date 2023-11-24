use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CheckoutCodeRequest {
    pub user: String,
    pub module: String,
    pub env: String,
}
