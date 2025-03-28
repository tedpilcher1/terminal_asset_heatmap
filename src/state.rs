use std::collections::HashMap;

use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, StatefulWidget, Widget},
};
use yahoo_finance_api::YahooConnector;

use crate::{assets::Asset, internal_terminal::InternalTerminalState, interval::Interval};

const LIGHT_GREEN: (f32, f32, f32) = (200.0, 255.0, 200.0);
const SOLID_GREEN: (f32, f32, f32) = (0.0, 255.0, 0.0);

const LIGHT_RED: (f32, f32, f32) = (255.0, 200.0, 200.0);
const SOLID_RED: (f32, f32, f32) = (255.0, 0.0, 0.0);

enum Gradient {
    Green,
    Red,
}

pub struct State {
    pub assets: HashMap<String, Asset>,
    pub internal_terminal: InternalTerminalState,
    pub interval: Interval,
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

impl State {
    pub fn new() -> Self {
        Self {
            assets: HashMap::new(),
            internal_terminal: InternalTerminalState::new(),
            interval: Interval::Day,
        }
    }

    pub async fn new_with_assets(provider: &mut YahooConnector) -> Self {
        let interval = Interval::Day;
        let mut assets = HashMap::new();

        if let Ok(amzn) = Asset::try_new("AMZN".to_owned(), provider, &interval).await {
            assets.insert("AMZN".to_owned(), amzn);
        }
        if let Ok(aapl) = Asset::try_new("AAPL".to_owned(), provider, &interval).await {
            assets.insert("AAPL".to_owned(), aapl);
        }
        if let Ok(googl) = Asset::try_new("GOOGL".to_owned(), provider, &interval).await {
            assets.insert("GOOGL".to_owned(), googl);
        }
        if let Ok(goog) = Asset::try_new("GOOG".to_owned(), provider, &interval).await {
            assets.insert("GOOG".to_owned(), goog);
        }
        if let Ok(pltr) = Asset::try_new("PLTR".to_owned(), provider, &interval).await {
            assets.insert("PLTR".to_owned(), pltr);
        }

        Self {
            assets,
            internal_terminal: InternalTerminalState::new(),
            interval,
        }
    }

    pub async fn update_interval(&mut self, new_interval: Interval, provider: &mut YahooConnector) {
        self.interval = new_interval;
        for (_, asset) in self.assets.iter_mut() {
            asset.update(provider, &self.interval).await.ok();
        }
    }
}

pub struct AssetHeatmapWidget {}

impl StatefulWidget for AssetHeatmapWidget {
    type State = HashMap<String, Asset>;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        let total_value: f64 = state
            .values()
            .map(|asset| asset.get_market_cap() as f64)
            .sum();

        if total_value == 0.0 || state.is_empty() {
            let block = Block::default()
                .title("No assets to display")
                .borders(Borders::ALL);
            block.render(area, buf);
            return;
        }

        let mut sorted_assets: Vec<&Asset> = state.values().collect();
        sorted_assets.sort_by(|a, b| {
            b.get_market_cap()
                .partial_cmp(&a.get_market_cap())
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        self.render_treemap(
            area,
            buf,
            &sorted_assets,
            total_value,
            Direction::Horizontal,
        );
    }
}

impl AssetHeatmapWidget {
    fn render_treemap(
        &self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        assets: &[&Asset],
        total_value: f64,
        direction: Direction,
    ) {
        if assets.is_empty() {
            return;
        }

        if assets.len() == 1 {
            self.render_asset_block(area, buf, assets[0]);
            return;
        }

        let mid = (assets.len() + 1) / 2;
        let first_group = &assets[..mid];
        let second_group = &assets[mid..];

        let first_value: f64 = first_group.iter().map(|a| a.get_market_cap() as f64).sum();
        let second_value: f64 = second_group.iter().map(|a| a.get_market_cap() as f64).sum();

        let first_percent = ((first_value / total_value) * 100.0) as u16;
        let second_percent = 100u16.saturating_sub(first_percent);

        let (first_percent, second_percent) = if first_percent < 5 && second_percent > 10 {
            (5, 95)
        } else if second_percent < 5 && first_percent > 10 {
            (95, 5)
        } else {
            (first_percent.max(1), second_percent.max(1))
        };

        let chunks = Layout::default()
            .direction(direction)
            .constraints([
                Constraint::Percentage(first_percent),
                Constraint::Percentage(second_percent),
            ])
            .split(area);

        let next_direction = match direction {
            Direction::Horizontal => Direction::Vertical,
            Direction::Vertical => Direction::Horizontal,
        };

        self.render_treemap(chunks[0], buf, first_group, first_value, next_direction);
        self.render_treemap(chunks[1], buf, second_group, second_value, next_direction);
    }

    fn render_asset_block(
        &self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        asset: &Asset,
    ) {
        let price = asset.get_price();
        let prev_price = asset.get_prev_price();

        let price_change_pct = if prev_price > 0.0 {
            ((price - prev_price) / prev_price) * 100.0
        } else {
            0.0
        };

        let color = if price_change_pct > 0.0 {
            let percentage = (price_change_pct.min(10.0) * 10.0) as f32;
            let rgb = Self::get_gradient(percentage, Gradient::Green);
            Color::Rgb(rgb.0, rgb.1, rgb.2)
        } else if price_change_pct < 0.0 {
            let percentage = (price_change_pct.abs().min(10.0) * 10.0) as f32;
            let rgb = Self::get_gradient(percentage, Gradient::Red);
            Color::Rgb(rgb.0, rgb.1, rgb.2)
        } else {
            Color::Rgb(100, 100, 100)
        };

        let block = Block::default()
            .title(format!(
                " {} ({:.2}%) ",
                asset.get_ticker(),
                price_change_pct
            ))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Black))
            .title_style(Style::default().fg(Color::Black))
            .style(Style::default().bg(color));

        block.render(area, buf);
    }

    fn get_gradient(percentage: f32, gradient: Gradient) -> (u8, u8, u8) {
        let percentage = percentage.clamp(0.0, 100.0);
        let (start_colour, end_colour) = match gradient {
            Gradient::Green => (LIGHT_GREEN, SOLID_GREEN),
            Gradient::Red => (LIGHT_RED, SOLID_RED),
        };

        let normalized = percentage / 100.0;

        let r = start_colour.0 + (end_colour.0 - start_colour.0) * normalized;
        let g = start_colour.1 + (end_colour.1 - start_colour.1) * normalized;
        let b = start_colour.2 + (end_colour.2 - start_colour.2) * normalized;

        (r.round() as u8, g.round() as u8, b.round() as u8)
    }
}
