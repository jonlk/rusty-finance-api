use crate::models::{BasicLiquidityRatio, BreakEvenPoint};

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
