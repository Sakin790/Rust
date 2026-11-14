use axum::{Json, Router, extract::State, routing::get};
use serde::Serialize;
use sqlx::{FromRow, PgPool};

// ১. ডেটাবেস থেকে ডেটা রিসিভ করার জন্য Struct (FromRow ম্যাক্রো দিয়ে ডেটাবেস রো সরাসরি ম্যাপ হবে)
#[derive(Serialize, FromRow)]
struct User {
    id: i32,
    name: String,
    email: String,
}

// ২. ডেটাবেস পুল হোল্ড করার জন্য অ্যাপ স্টেট (App State)
#[derive(Clone)]
struct AppState {
    db: PgPool,
}

#[tokio::main]
async fn main() {
    // .env ফাইল লোড করা (যাতে ডেটাবেস URL পড়া যায়)
    dotenvy::dotenv().ok();
    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable must be set");

    // ডেটাবেস কানেকশন পুল তৈরি করা
    let db_pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to Postgres database!");

    println!("✅ Connected to PostgreSQL database successfully!");

    // স্টেট এর মধ্যে পুল পাস করা
    let state = AppState { db: db_pool };

    // ৩. রাউটার তৈরি এবং স্টেট শেয়ার করা
    let app = Router::new()
        .route("/users", get(get_users))
        .with_state(state); // .with_state দিয়ে পুরো অ্যাপ্লিকেশনে ডেটাবেস পুল ছড়িয়ে দেওয়া হলো

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("🚀 Server is running at http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}

// ৪. হ্যান্ডলার ফাংশন (যেখানে স্টেট থেকে ডেটাবেস পুল রিসিভ করা হচ্ছে)
async fn get_users(State(state): State<AppState>) -> Json<Vec<User>> {
    // ডেটাবেস থেকে কুয়েরি করে ইউজারদের নিয়ে আসা
    let users = sqlx::query_as::<_, User>("SELECT id, name, email FROM users")
        .fetch_all(&state.db)
        .await
        .unwrap_or_default();

    Json(users)
}
