use crate::models::BasicLiquidityRatio;

pub trait Calculation {
    fn calculate(&mut self);
}

impl Calculation for BasicLiquidityRatio {
    fn calculate(&mut self) {
        self.result = Some(self.monetary_assets - self.monthly_expenses);
    }
}
