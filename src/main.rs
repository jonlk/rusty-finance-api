mod finance;
mod models;
use axum::{extract::Query, routing::get, Json, Router};
use models::{
    BasicLiquidityRatio, BreakEvenPoint, CashFlow, CompoundInterest, NetIncome, NetWorth, PERatio,
    RuleOf72, SimpleInterest, VariationOfInvestment,
};

use crate::finance::Calculation;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/basicliquidityratio",get(create_response::<BasicLiquidityRatio>))
        .route("/breakevenpoint", get(create_response::<BreakEvenPoint>))
        .route("/cashflow", get(create_response::<CashFlow>))
        .route("/compoundinterest",get(create_response::<CompoundInterest>))
        .route("/ruleof72", get(create_response::<RuleOf72>))
        .route("/netincome", get(create_response::<NetIncome>))
        .route("/networth", get(create_response::<NetWorth>))
        .route("/peratio", get(create_response::<PERatio>))
        .route("/simpleinterest", get(create_response::<SimpleInterest>))
        .route("/variationofinvestment",get(create_response::<VariationOfInvestment>));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn create_response<T: Calculation>(query: Query<T>) -> Json<T> {
    let mut response = query.0;
    response.calculate();
    Json(response)
}
