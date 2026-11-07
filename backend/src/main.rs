use tower_http::cors::{Any, CorsLayer};

use axum::{
    extract::Multipart,
    http::{Method, StatusCode},
    response::IntoResponse,
    routing::post,
    Router,
};

#[tokio::main]
async fn main() {

    let cors = CorsLayer::new()
    // Дозволити запити з будь-яких адрес
    .allow_origin(Any)
    .allow_methods([Method::GET, Method::POST])
    .allow_headers(Any);

    let app = Router::new()
        .route("/upload", post(upload_csv))
        .layer(cors);

    println!("Працює на http://localhost:3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn upload_csv(mut multipart: Multipart) -> Result<impl IntoResponse, (StatusCode, String)> {
    while let Some(field) = multipart.next_field().await.unwrap_or(None) {
        if field.name() == Some("file") {
            // Витягування сирих байтів файлу
            let data = field.bytes().await.map_err(|e| {
                (StatusCode::BAD_REQUEST, format!("Помилка читання: {}", e))
            })?;

            // Закидання байтів в CSV парсер
            let mut rdr = csv::Reader::from_reader(data.as_ref());
            let mut records: Vec<std::collections::HashMap<String, String>> = Vec::new();

            // Читання рядків
            for result in rdr.deserialize() {
                match result {
                    Ok(record) => records.push(record),
                    Err(e) => return Err((StatusCode::BAD_REQUEST, format!("Кривий CSV: {}", e))),
                }
            }

            // Відправка даних в пайтон сервіс
            let client = reqwest::Client::new();
            let py_response = client
                .post("http://localhost:8000/analyze")
                .json(&serde_json::json!({ "data": records }))
                .send()
                .await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Помилка Python сервісу: {}", e)))?;

            // Отримання результату від пайтона
            let final_json: serde_json::Value = py_response
                .json()
                .await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Помилка парсингу відповіді: {}", e)))?;

            // Відправка готового результату на Vue фронтенд
            return Ok(axum::Json(final_json));
        }
    }

    Err((StatusCode::BAD_REQUEST, "Файл не знайдено".into()))
}