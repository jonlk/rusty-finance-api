use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BasicLiquidityRatio {
    pub monetary_assets: f64,
    pub monthly_expenses: f64,
    pub result: Option<f64>,
}

#[derive(Serialize, Deserialize)]
pub struct BreakEvenPoint {
    pub gross_profit_margin: f64,
    pub fixed_expenses: f64,
    pub result: Option<f64>,
}

#[derive(Serialize, Deserialize)]
pub struct CashFlow {
    pub income: f64,
    pub expenses: f64,
    pub result: Option<f64>,
}

#[derive(Serialize, Deserialize)]
pub struct CompoundInterest {
    pub principal: f64,
    pub annual_interest_rate: f64,
    pub times_cmpd_per_year: f64,
    pub length_borrowed_years: f64,
    pub result: Option<f64>,
}

#[derive(Serialize, Deserialize)]
pub struct RuleOf72 {
    pub compound_interest_rate: f64,
    pub result: Option<f64>,
}
