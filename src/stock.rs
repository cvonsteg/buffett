pub struct Stock {
    // Core struct to represent a stock with OHLC data
    pub name: String,
    pub price_open: f32,
    pub price_high: f32,
    pub price_low: f32,
    pub price_close: f32
}

impl Stock {
    pub fn day_delta(&self) -> f32 {
        self.price_close - self.price_open
    }

    pub fn day_delta_perc(&self) -> f32 {
        self.day_delta() / self.price_open
    }

    pub fn max_delta(&self) -> f32 {
        self.price_high - self.price_low
    }

    pub fn output_stats(&self) -> String {
        format!("{}, {:.3}, {:.3}, {:.3}", self.name, self.day_delta(), self.day_delta_perc(), self.max_delta())
    }
}
