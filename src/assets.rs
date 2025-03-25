use anyhow::{Ok, Result, bail};
use yahoo_finance_api::YahooConnector;

use crate::state::Interval;

pub struct Asset {
    ticker: String,
    price: f64,
    prev_price: f64,
}

impl Asset {
    pub async fn try_new(
        ticker: String,
        provider: &mut YahooConnector,
        interval: &Interval,
    ) -> Result<Self> {
        let (prev_price, price) = Asset::pull_update(&ticker, provider, interval).await?;

        Ok(Self {
            ticker,
            price,
            prev_price,
        })
    }

    pub fn get_ticker(&self) -> &str {
        &self.ticker
    }

    pub fn get_price(&self) -> f64 {
        self.price
    }

    pub fn get_percentage_price_change(&self) -> f64 {
        ((self.price - self.prev_price) / self.prev_price) * 100.0
    }

    pub async fn update(
        &mut self,
        provider: &mut YahooConnector,
        interval: &Interval,
    ) -> Result<()> {
        let (prev_price, price) = Asset::pull_update(&self.ticker, provider, interval).await?;
        self.prev_price = prev_price;
        self.price = price;

        Ok(())
    }

    /// returns (prev_price, price)
    async fn pull_update(
        ticker: &str,
        provider: &mut YahooConnector,
        interval: &Interval,
    ) -> Result<(f64, f64)> {
        let internal_string: String = interval.into();
        let response = provider.get_latest_quotes(ticker, &internal_string).await?;
        let quotes = response.quotes()?;

        match (quotes.get(quotes.len() - 2), quotes.last()) {
            (Some(second_last_quote), Some(last_quote)) => {
                Ok((second_last_quote.close, last_quote.close))
            }
            _ => bail!("Failed to get second last and last price"),
        }
    }
}
