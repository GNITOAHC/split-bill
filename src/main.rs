use axum::{extract::Query, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use serde_json::json;
use std::collections::HashMap;

mod graph;
use graph::DirectedGraph;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/calc-balance", get(calc_balance));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();
    println!("Listening on: http://localhost:8000");
    axum::serve(listener, app).await.unwrap();
}

async fn calc_balance(Query(params): Query<Vec<(String, String)>>) -> impl IntoResponse {
    let mut map: HashMap<String, f32> = HashMap::new();
    for (key, value) in params {
        match value.parse::<f64>() {
            Ok(val) => {
                let entry = map.entry(key).or_insert(0.0);
                *entry += val as f32;
            }
            Err(e) => {
                return (StatusCode::BAD_REQUEST, e.to_string()).into_response();
            }
        }
    }

    let mut dgraph: DirectedGraph<String, u32> = DirectedGraph::new();

    for (key, value) in map {
        let parts: Vec<&str> = key.split(':').collect();
        dgraph.add_edge(parts[0].to_string(), parts[1].to_string(), value as u32);
    }

    dgraph.display();

    (StatusCode::OK, Json(json!(dgraph))).into_response()
}
