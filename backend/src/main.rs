use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;
use std::net::SocketAddr;
use std::collections::HashMap;
use tower_http::cors::{Any, CorsLayer};

use axum::{
    extract::{Multipart, State},
    http::{Method, StatusCode},
    response::IntoResponse,
    routing::post,
    Json, Router,
};

pub mod routes;

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
}

#[tokio::main]
async fn main() {

    dotenvy::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("Не знайдено DATABASE_URL");

    let pool = match SqlitePoolOptions::new()
    .max_connections(5)
    .connect(&db_url)
    .await
    {
        Ok(pool) => {
            println!("Успішно під'єднано до БД");
            pool
        }
        Err(err) => {
            println!("Помилка при підключенні до БД: {}", err);
            std::process::exit(1);
        }
    };

    let state = AppState { db: pool };


    let cors = CorsLayer::new()
    // Дозволити запити з будь-яких адрес
    .allow_origin(Any)
    .allow_methods([Method::GET, Method::POST, Method:: PUT, Method::PATCH, Method::DELETE])
    .allow_headers(Any);

    let app = Router::new()
        .route("/upload", post(upload_csv))
        .layer(cors)
        .with_state(state);

    println!("Працює http://localhost:3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn upload_csv( State(state): State<AppState>, mut multipart: Multipart) -> Result<impl IntoResponse, (StatusCode, String)> {
    // Пошук поля "file" з Vue
    while let Some(field) = multipart.next_field().await.unwrap_or(None) {
        if field.name() == Some("file") {
            // Витягування сирих байтів файлу
            let data = field.bytes().await.map_err(|e| {
                (StatusCode::BAD_REQUEST, format!("Помилка читання: {}", e))
            })?;

            // Закидання байтів в CSV парсер
            let mut rdr = csv::Reader::from_reader(data.as_ref());
            let mut records: Vec<HashMap<String, String>> = Vec::new();

            // Читання рядків
            for result in rdr.deserialize() {
                match result {
                    Ok(record) => records.push(record),
                    Err(e) => return Err((StatusCode::BAD_REQUEST, format!("Кривий CSV: {}", e))),
                }
            }

            let response = serde_json::json!({
                "status": "ok",
                "rows_processed": records.len(),
                "data": records
            });

            return Ok(Json(response));
        }
    }

    Err((StatusCode::BAD_REQUEST, "Файл не знайдено".into()))
}