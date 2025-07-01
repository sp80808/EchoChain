use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

// Data Models
#[derive(Debug, Serialize, Deserialize)]
struct User {
    user_id: String,
    username: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Sample {
    id: String,
    title: String,
    artist: String,
    duration: i32,
    category: String,
    p2p_content_id: String,
    price: f64,
    owner_address: String,
    blockchain_hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct AuthResponse {
    token: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ErrorResponse {
    message: String,
}

// App State
struct AppState {
    samples: Mutex<Vec<Sample>>,
    users: Mutex<Vec<User>>,
}

// Handlers
async fn register_user(
    user_data: web::Json<User>,
    data: web::Data<AppState>,
) -> impl Responder {
    let mut users = data.users.lock().unwrap();
    users.push(user_data.into_inner());
    HttpResponse::Ok().json(users.last().unwrap())
}

async fn login_user() -> impl Responder {
    HttpResponse::Ok().json(AuthResponse {
        token: "sample_jwt_token".to_string(),
    })
}

async fn list_samples(
    query: web::Query<SampleQuery>,
    data: web::Data<AppState>,
) -> impl Responder {
    let samples = data.samples.lock().unwrap();
    HttpResponse::Ok().json(samples.clone())
}

async fn register_sample(
    sample_data: web::Json<Sample>,
    data: web::Data<AppState>,
) -> impl Responder {
    let mut samples = data.samples.lock().unwrap();
    samples.push(sample_data.into_inner());
    HttpResponse::Created().json(samples.last().unwrap())
}

async fn check_copyright() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "infringementStatus": "clean",
        "details": "No copyright issues detected"
    }))
}

// Query Params
#[derive(Debug, Deserialize)]
struct SampleQuery {
    search: Option<String>,
    category: Option<String>,
}

// Main
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        samples: Mutex::new(Vec::new()),
        users: Mutex::new(Vec::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(
                web::scope("/api")
                    .route("/register", web::post().to(register_user))
                    .route("/login", web::post().to(login_user))
                    .route("/samples", web::get().to(list_samples))
                    .route("/samples", web::post().to(register_sample))
                    .route("/copyright-check", web::post().to(check_copyright))
            )
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}