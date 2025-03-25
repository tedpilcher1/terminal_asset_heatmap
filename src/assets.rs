use anyhow::{Ok, Result, bail};
use yahoo_finance_api::YahooConnector;

use crate::interval::Interval;

pub struct Asset {
    ticker: String,
    price: f64,
    prev_price: f64,
    market_cap: u64,
}

impl Asset {
    pub async fn try_new(
        ticker: String,
        provider: &mut YahooConnector,
        interval: &Interval,
    ) -> Result<Self> {
        let (prev_price, price, market_cap) =
            Asset::pull_update(&ticker, provider, interval).await?;

        Ok(Self {
            ticker,
            price,
            prev_price,
            market_cap,
        })
    }

    pub fn get_ticker(&self) -> &str {
        &self.ticker
    }

    pub fn get_market_cap(&self) -> u64 {
        self.market_cap
    }

    pub fn get_price(&self) -> f64 {
        self.price
    }

    pub fn get_prev_price(&self) -> f64 {
        self.prev_price
    }

    pub fn get_percentage_price_change(&self) -> f64 {
        ((self.price - self.prev_price) / self.prev_price) * 100.0
    }

    pub async fn update(
        &mut self,
        provider: &mut YahooConnector,
        interval: &Interval,
    ) -> Result<()> {
        let (prev_price, price, mcap) = Asset::pull_update(&self.ticker, provider, interval)
            .await
            .unwrap();

        self.prev_price = prev_price;
        self.price = price;
        self.market_cap = mcap;

        Ok(())
    }

    /// returns (prev_price, price, market_cap)
    async fn pull_update(
        ticker: &str,
        provider: &mut YahooConnector,
        interval: &Interval,
    ) -> Result<(f64, f64, u64)> {
        let response = provider
            .get_latest_quotes(ticker, interval.to_provider_format())
            .await?;
        let quotes = response
            .quotes()
            .map_err(|e| anyhow::anyhow!("Failed to extract quotes: {}", e))?;

        if quotes.len() < 2 {
            bail!("Insufficient price data available, need at least 2 quotes");
        }
        let ticker_info = provider.get_ticker_info(ticker).await?;

        let market_cap = ticker_info
            .quote_summary
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Missing quote summary"))?
            .result
            .first()
            .ok_or_else(|| anyhow::anyhow!("Empty result in quote summary"))?
            .summary_detail
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Missing summary detail"))?
            .market_cap
            .ok_or_else(|| anyhow::anyhow!("Missing market cap"))?;

        let second_last_quote = &quotes[quotes.len() - 2];
        let last_quote = quotes.last().unwrap();

        Ok((second_last_quote.close, last_quote.close, market_cap))
    }
}
