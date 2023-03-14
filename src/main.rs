mod controller;
mod finance;
mod models;
use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route(
            "/basicliquidityratio",
            get(controller::get_basic_liquidity_ratio),
        )
        .route("/breakevenpoint", get(controller::get_break_even_point))
        .route("/compoundinterest", get(controller::get_compound_interest));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
