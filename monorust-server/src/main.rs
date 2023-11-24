use axum::{
    routing::{get, post},
    Router, response::IntoResponse,
    extract::{Json, State}
};
use monorust_models::CheckoutCodeRequest;
use sqlx::{Pool, Sqlite, prelude::FromRow};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()>{

    let pool = sqlx::SqlitePool::connect("sqlite::memory:").await?;
    sqlx::migrate!("../migrations")
        .run(&pool)
        .await?;

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/checkout", post(checkout_code))
        .with_state(pool);

    println!("running on http://localhost:3000");
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

#[derive(FromRow, Debug)]
struct Checkout {
    id: i64,
    module: String,
    environment: String,
    user: String,
}

async fn get_checkouts(State(pool): State<Pool<Sqlite>>, Json(payload): Json<CheckoutCodeRequest>) -> impl IntoResponse {
    let pool = pool.clone();
    
    let _ = sqlx::query("INSERT INTO checkouts (module, environment, user) VALUES (?1, ?2, ?3)")
        .bind(payload.module)
        .bind(payload.env)
        .bind(payload.user)
        .execute(&pool)
        .await
        .unwrap();

    let checkouts: Vec<Checkout> = sqlx::query_as("SELECT id, module, environment, user FROM checkouts")
        .fetch_all(&pool)
        .await
        .unwrap();

    println!("Db has stuff: {}", checkouts.len());
    format!("db stuff {}", checkouts.len())
}

async fn checkout_code(State(pool): State<Pool<Sqlite>>, Json(payload): Json<CheckoutCodeRequest>) -> impl IntoResponse {
    let pool = pool.clone();
    
    let _ = sqlx::query("INSERT INTO checkouts (module, environment, user) VALUES (?1, ?2, ?3)")
        .bind(payload.module)
        .bind(payload.env)
        .bind(payload.user)
        .execute(&pool)
        .await
        .unwrap();

    let checkouts: Vec<Checkout> = sqlx::query_as("SELECT id, module, environment, user FROM checkouts")
        .fetch_all(&pool)
        .await
        .unwrap();

    println!("Db has stuff: {}", checkouts.len());
    format!("db stuff {}", checkouts.len())
}