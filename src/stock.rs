pub struct Stock {
    pub name: String,
    pub price_open: f64,
    pub price_high: f64,
    pub price_low: f64,
    pub price_close: f64
}

impl Stock {
    pub fn day_delta(&self) -> f64 {
        self.price_close - self.price_open
    }

    pub fn day_delta_perc(&self) -> f64 {
        self.day_delta() / self.price_open
    }

    pub fn max_delta(&self) -> f64 {
        self.price_high - self.price_low
    }

    pub fn output_stats(&self) -> String {
        format!("{}, {:.3}, {:.3}, {:.3}", self.name, self.day_delta(), self.day_delta_perc(), self.max_delta())
    }
}