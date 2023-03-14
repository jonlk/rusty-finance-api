use axum::{extract::Query, Json};

use crate::{
    finance::Calculation,
    models::{BasicLiquidityRatio, BreakEvenPoint},
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

//TODO: Figure out how to route all responses to one handler
// pub async fn create_response<T: Calculation>(model: &mut T) -> Json<&mut T> {
//     model.calculate();
//     Json(model)
// }
