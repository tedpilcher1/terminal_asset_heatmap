use anyhow::{Ok, Result};
use yahoo_finance_api::YahooConnector;

pub struct Asset {
    ticker: String,
    price: f64,
    prev_price: f64,
}

impl Asset {
    pub fn new(ticker: String) -> Self {
        Self {
            ticker,
            price: 0.0,
            prev_price: 0.0,
        }
    }

    pub fn get_price(&self) -> f64 {
        self.price
    }

    pub fn get_percentage_price_change(&self) -> f64 {
        ((self.price - self.prev_price) / self.prev_price) * 100.0
    }

    pub async fn update(&mut self, provider: &mut YahooConnector, interval: &str) -> Result<()> {
        let response = provider.get_latest_quotes(&self.ticker, interval).await?;
        let quotes = response.quotes()?;

        if let Some(last_quote) = quotes.last() {
            self.price = last_quote.close;
        }

        if let Some(second_last_quote) = quotes.get(quotes.len() - 2) {
            self.prev_price = second_last_quote.close;
        }

        Ok(())
    }
}
