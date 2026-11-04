use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;
use std::net::SocketAddr;

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




    println!("Hello, world!");


    let state = AppState { db: pool };

    let app = routes::create_router(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Сервер запущений за адресою: http://localhost:3000");
}