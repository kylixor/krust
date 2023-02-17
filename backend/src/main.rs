use actix_web::{web, App, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};
use sqlx::mysql::{MySqlPool, MySqlPoolOptions, MySqlQueryResult};

#[derive(Clone)]
struct AppState {
    pool: MySqlPool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    const DB_URL: &str = "mysql://user:password@127.0.0.1:3306/sqlx";

    let pool: MySqlPool = MySqlPoolOptions::new()
        .max_connections(10)
        .connect(DB_URL)
        .await
        .unwrap();

    let app_state = AppState { pool };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .route("/", web::get().to(root))
            .route("/get/{user_id}", web::get().to(get_user))
            .route("/get", web::get().to(get_users))
            .route("/delete/{user_id}", web::delete().to(delete_user))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}

async fn root() -> String {
    "Server is up and running".to_string()
}

#[derive(Serialize, Deserialize)]
struct User {
    id: i32,
    username: String,
    email: String,
}

async fn get_user(path: web::Path<usize>, app_state: web::Data<AppState>) -> HttpResponse {
    let user_id: usize = path.into_inner();

    let user: Option<User> =
        sqlx::query_as!(User, "SELECT * FROM users WHERE id = ?", user_id as u64,)
            .fetch_optional(&app_state.pool)
            .await
            .unwrap();

    match user {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::BadRequest().into(),
    }
}

async fn delete_user(path: web::Path<usize>, app_state: web::Data<AppState>) -> HttpResponse {
    let user_id: usize = path.into_inner();

    let deleted: sqlx::Result<MySqlQueryResult> =
        sqlx::query!("DELETE FROM users WHERE id = ?", user_id as u64)
            .execute(&app_state.pool)
            .await;

    match deleted {
        Ok(_) => HttpResponse::Ok().into(),
        Err(_) => HttpResponse::BadRequest().into(),
    }
}

async fn get_users(app_state: web::Data<AppState>) -> HttpResponse {
    match sqlx::query_as!(User, "SELECT * FROM users",)
        .fetch_all(&app_state.pool)
        .await
    {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}
