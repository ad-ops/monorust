use anyhow::Result;
use monorust_models::CheckoutCodeRequest;

pub fn say_hello() -> Result<String> {
    let body = reqwest::blocking::get("http://localhost:3000")?.text()?;

    Ok(body)
}

pub fn checkout_code(user: &str, module: &str, env: &str) -> Result<String> {
    let client = reqwest::blocking::Client::new();
    let body = CheckoutCodeRequest {
        user: user.to_string(),
        module: module.to_string(),
        env: env.to_string(),
    };

    let response = client
        .post("http://localhost:3000/checkout")
        .json(&body)
        .send()?;

    let status = response.status().to_string();

    Ok(status)
}
