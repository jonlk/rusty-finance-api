use crate::models::{BasicLiquidityRatio, BreakEvenPoint, CashFlow, CompoundInterest};

pub trait Calculation {
    fn calculate(&mut self);
}

impl Calculation for BasicLiquidityRatio {
    fn calculate(&mut self) {
        self.result = Some(self.monetary_assets - self.monthly_expenses);
    }
}

impl Calculation for BreakEvenPoint {
    fn calculate(&mut self) {
        self.result = Some(self.fixed_expenses / self.gross_profit_margin);
    }
}

impl Calculation for CashFlow {
    fn calculate(&mut self) {
        self.result = Some(self.income - self.expenses);
    }
}

impl Calculation for CompoundInterest {
    fn calculate(&mut self) {
        let pow = self.times_cmpd_per_year * self.length_borrowed_years;
        let rate_div = self.annual_interest_rate / self.times_cmpd_per_year;
        let result = self.principal*(f64::powf(1.0 + rate_div, pow));
        self.result = Some(result)
    }
}
