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

#[derive(Serialize, Deserialize)]
pub struct NetIncome {
    pub revenue: f64,
    pub expenses: f64,
    pub result: Option<f64>,
}

#[derive(Serialize, Deserialize)]
pub struct NetWorth {
    pub assets: f64,
    pub debts: f64,
    pub result: Option<f64>,
}

#[derive(Serialize, Deserialize)]
pub struct PERatio {
    pub price_per_share: f64,
    pub earnings_per_share: f64,
    pub result: Option<f64>,
}

#[derive(Serialize, Deserialize)]
pub struct SimpleInterest {
    pub principal: f64,
    pub annual_interest_rate: f64,
    pub length_borrowed_in_years: f64,
    pub result: Option<f64>,
}

#[derive(Serialize, Deserialize)]
pub struct VariationOfInvestment {
    pub current_price: f64,
    pub purchase_price: f64,
    pub result: Option<f64>,
}
