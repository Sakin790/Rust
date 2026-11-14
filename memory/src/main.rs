use axum::{http::StatusCode, routing::get, Json, Router};
use serde::Serialize;

#[derive(Serialize)]
struct ApiResponse {
    success: bool,
    message: String,
    items_count: usize,
}

#[derive(Serialize)]
struct Product {
    id: i32,
    name: String,
    quantity: i32,
    isstock: bool,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/api/data", get(get_json_handler))
        .route("/api/product", get(fetch_product));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Server is running at http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}

async fn get_json_handler() -> (StatusCode, Json<ApiResponse>) {
    let data = ApiResponse {
        success: true,
        message: "Hello from Rust Axum server! Data fetched successfully.".to_string(),
        items_count: 110,
    };

    (StatusCode::CREATED, Json(data))
}

async fn fetch_product() -> Json<Product> {
    let data = Product {
        id: 1,
        isstock: true,
        name: String::from("Tshirt"),
        quantity: 23,
    };
    Json(data)
}
