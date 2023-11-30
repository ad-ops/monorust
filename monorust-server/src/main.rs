use anyhow::Result;
use axum::{
    extract::{Json, State},
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use monorust_models::{Checkout, CheckoutCodeRequest};
use sqlx::{Pool, Sqlite, FromRow};

#[tokio::main]
async fn main() -> Result<()> {
    let pool = sqlx::SqlitePool::connect("sqlite::memory:").await?;
    sqlx::migrate!("../migrations").run(&pool).await?;

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/checkout", post(checkout_code).get(get_checkouts))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("running on http://localhost:3000");
    axum::serve(listener, app.into_make_service());

    Ok(())
}

#[derive(FromRow, Debug)]
pub struct CheckoutData {
    pub id: i64,
    pub module: String,
    pub environment: String,
    pub user: String,
}

async fn get_checkouts(State(pool): State<Pool<Sqlite>>) -> impl IntoResponse {
    let pool = pool.clone();

    let checkouts: Vec<CheckoutData> =
        sqlx::query_as("SELECT id, module, environment, user FROM checkouts")
            .fetch_all(&pool)
            .await
            .unwrap();

    println!("Db has stuff: {}", checkouts.len());
    let checkouts: Vec<Checkout> = checkouts
        .into_iter()
        .map(|data| Checkout { 
            id: data.id,
            module: data.module,
            environment: data.environment,
            user: data.user,
        })
        .collect();
    Json(checkouts)
}

async fn checkout_code(
    State(pool): State<Pool<Sqlite>>,
    Json(payload): Json<CheckoutCodeRequest>,
) -> impl IntoResponse {
    let pool = pool.clone();

    let _ = sqlx::query("INSERT INTO checkouts (module, environment, user) VALUES (?1, ?2, ?3)")
        .bind(payload.module)
        .bind(payload.env)
        .bind(payload.user)
        .execute(&pool)
        .await
        .unwrap();

    let checkouts: Vec<CheckoutData> =
        sqlx::query_as("SELECT id, module, environment, user FROM checkouts")
            .fetch_all(&pool)
            .await
            .unwrap();

    println!("Db has stuff: {}", checkouts.len());
    format!("db stuff {}", checkouts.len())
}
