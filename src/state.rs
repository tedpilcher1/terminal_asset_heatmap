use std::collections::HashMap;

use crate::{assets::Asset, internal_terminal::InternalTerminalState};

#[derive(Clone)]
pub enum Interval {
    Day,
    Week,
    Month,
    ThreeMonth,
    SixMonth,
    Year,
    FiveYear,
}

impl TryFrom<String> for Interval {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "1d" => Ok(Self::Day),
            "1w" => Ok(Self::Week),
            "1m" => Ok(Self::Month),
            "3m" => Ok(Self::ThreeMonth),
            "6m" => Ok(Self::SixMonth),
            "1y" => Ok(Self::Year),
            "5y" => Ok(Self::FiveYear),
            _ => Err(()),
        }
    }
}

impl From<&Interval> for String {
    fn from(interval: &Interval) -> Self {
        match interval {
            Interval::Day => "1d".to_string(),
            Interval::Week => "1w".to_string(),
            Interval::Month => "1m".to_string(),
            Interval::ThreeMonth => "3m".to_string(),
            Interval::SixMonth => "6m".to_string(),
            Interval::Year => "1y".to_string(),
            Interval::FiveYear => "5y".to_string(),
        }
    }
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
}
