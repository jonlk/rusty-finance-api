use axum::{extract::Query, Json};

use crate::{finance::Calculation, models::BasicLiquidityRatio};

pub async fn get_basic_liquidity_ratio(
    blr: Query<BasicLiquidityRatio>,
) -> Json<BasicLiquidityRatio> {
    let mut test: BasicLiquidityRatio = blr.0;
    test.calculate();
    Json(test)
}
