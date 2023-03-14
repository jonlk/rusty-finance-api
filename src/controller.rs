use axum::{extract::Query, Json};

use crate::{
    finance::Calculation,
    models::{BasicLiquidityRatio, BreakEvenPoint, CompoundInterest},
};

pub async fn get_basic_liquidity_ratio(
    query: Query<BasicLiquidityRatio>,
) -> Json<BasicLiquidityRatio> {
    create_response(query.0).await
}

pub async fn get_break_even_point(query: Query<BreakEvenPoint>) -> Json<BreakEvenPoint> {
    create_response(query.0).await
}

pub async fn get_compound_interest(query: Query<CompoundInterest>) -> Json<CompoundInterest> {
    create_response(query.0).await
}

//TODO: Figure out how to route all responses to one handler
async fn create_response<T: Calculation>(model: T) -> Json<T> {
    let mut response = model;
    response.calculate();
    Json(response)
}
