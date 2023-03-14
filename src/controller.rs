use axum::{extract::Query, Json};

use crate::{
    finance::Calculation,
    models::{BasicLiquidityRatio, BreakEvenPoint, CompoundInterest},
};

pub async fn get_basic_liquidity_ratio(
    query: Query<BasicLiquidityRatio>,
) -> Json<BasicLiquidityRatio> {
    let mut model = query.0;
    model.calculate();
    Json(model)
}

pub async fn get_break_even_point(query: Query<BreakEvenPoint>) -> Json<BreakEvenPoint> {
    let mut model = query.0;
    model.calculate();
    Json(model)
}

pub async fn get_compound_interest(query: Query<CompoundInterest>) -> Json<CompoundInterest> {
    let mut model = query.0;
    model.calculate();
    Json(model)
}
