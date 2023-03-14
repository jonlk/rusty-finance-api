mod finance;
mod models;
use axum::{extract::Query, routing::get, Json, Router};
use models::{BasicLiquidityRatio, BreakEvenPoint, CompoundInterest};

use crate::finance::Calculation;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/basicliquidityratio",get(create_response::<BasicLiquidityRatio>))
        .route("/breakevenpoint", get(create_response::<BreakEvenPoint>))
        .route("/compoundinterest",get(create_response::<CompoundInterest>));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

pub async fn create_response<T: Calculation>(query: Query<T>) -> Json<T> {
    let mut response = query.0;
    response.calculate();
    Json(response)
}
