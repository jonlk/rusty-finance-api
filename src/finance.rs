use crate::models::{
    BasicLiquidityRatio, BreakEvenPoint, CashFlow, CompoundInterest, NetIncome, NetWorth, PERatio,
    RuleOf72, SimpleInterest, VariationOfInvestment,
};

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
        self.result = Some(
            self.principal
                * (f64::powf(
                    1.0 + (self.annual_interest_rate / self.times_cmpd_per_year),
                    self.times_cmpd_per_year * self.length_borrowed_years,
                )),
        )
    }
}

impl Calculation for RuleOf72 {
    fn calculate(&mut self) {
        self.result = Some(72.0 / self.compound_interest_rate);
    }
}

impl Calculation for NetIncome {
    fn calculate(&mut self) {
        self.result = Some(self.revenue - self.expenses);
    }
}

impl Calculation for NetWorth {
    fn calculate(&mut self) {
        self.result = Some(self.assets - self.debts);
    }
}

impl Calculation for PERatio {
    fn calculate(&mut self) {
        self.result = Some(self.price_per_share / self.earnings_per_share);
    }
}

impl Calculation for SimpleInterest {
    fn calculate(&mut self) {
        self.result =
            Some(self.principal * self.annual_interest_rate * self.length_borrowed_in_years);
    }
}

impl Calculation for VariationOfInvestment {
    fn calculate(&mut self) {
        self.result = Some((self.current_price - self.purchase_price) / self.purchase_price);
    }
}
