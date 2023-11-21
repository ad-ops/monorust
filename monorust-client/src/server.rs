use anyhow::Result;

pub fn say_hello() -> Result<String> {
    let body = reqwest::blocking::get("http://localhost:3000")?
        .text()?;

    Ok(body)
}