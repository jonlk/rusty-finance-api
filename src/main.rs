mod finance;
mod models;

use axum::{extract::Query, routing::get, Json, Router};
use chrono::Utc;
use serde::{Deserialize, Serialize};

use models::{
    BasicLiquidityRatio, BreakEvenPoint, CashFlow, CompoundInterest, NetIncome, NetWorth, PERatio,
    RuleOf72, SimpleInterest, VariationOfInvestment,
};
use uuid::Uuid;

use crate::finance::Calculation;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", get(health_check))
        .route(
            "/basicliquidityratio",
            get(create_response::<BasicLiquidityRatio>),
        )
        .route("/breakevenpoint", get(create_response::<BreakEvenPoint>))
        .route("/cashflow", get(create_response::<CashFlow>))
        .route(
            "/compoundinterest",
            get(create_response::<CompoundInterest>),
        )
        .route("/ruleof72", get(create_response::<RuleOf72>))
        .route("/netincome", get(create_response::<NetIncome>))
        .route("/networth", get(create_response::<NetWorth>))
        .route("/peratio", get(create_response::<PERatio>))
        .route("/simpleinterest", get(create_response::<SimpleInterest>))
        .route(
            "/variationofinvestment",
            get(create_response::<VariationOfInvestment>),
        );

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn create_response<T: Calculation>(query: Query<T>) -> Json<ResponseModel<T>> {
    let mut response = query.0;
    response.calculate();
    let response_model = ResponseModel {
        id: String::from(Uuid::new_v4().to_string()),
        timestamp: String::from(Utc::now().to_string()),
        data: response,
    };
    Json(response_model)
}

async fn health_check() -> String {
    println!("{:?} | healthy", Utc::now().to_string());
    String::from("healthy")
}

#[derive(Serialize, Deserialize)]
struct ResponseModel<T: Calculation> {
    id: String,
    timestamp: String,
    data: T,
}
