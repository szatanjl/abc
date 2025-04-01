mod data;

use std::sync::{Arc, RwLock};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    serve, Json, Router,
};
use tokio::net::TcpListener;
use self::data::DataPoints;

async fn add_batch(
    Path(symbol): Path<String>,
    State(data): State<Arc<RwLock<DataPoints>>>,
    Json(values): Json<Vec<f64>>,
) {
    let mut data = data.write().unwrap();
    data.add(symbol, &values);
}

async fn stats(
    Path((symbol, k)): Path<(String, u8)>,
    State(data): State<Arc<RwLock<DataPoints>>>,
) -> Result<impl IntoResponse, StatusCode> {
    let data = data.read().unwrap();
    data.get(&symbol, k).map(Json).map_err(|_| StatusCode::BAD_REQUEST)
}

#[tokio::main]
async fn main() {
    let data = Arc::new(RwLock::new(DataPoints::default()));
    let api = Router::new()
        .route("/add_batch/{symbol}", post(add_batch))
        .route("/stats/{symbol}/{k}", get(stats))
        .with_state(data);
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    serve(listener, api).await.unwrap();
}
