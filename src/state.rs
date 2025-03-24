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
            "1D" => Ok(Self::Day),
            "1W" => Ok(Self::Week),
            "1M" => Ok(Self::Month),
            "3M" => Ok(Self::ThreeMonth),
            "6M" => Ok(Self::SixMonth),
            "1Y" => Ok(Self::Year),
            "5Y" => Ok(Self::FiveYear),
            _ => Err(()),
        }
    }
}

impl From<Interval> for String {
    fn from(interval: Interval) -> Self {
        match interval {
            Interval::Day => "1D".to_string(),
            Interval::Week => "1W".to_string(),
            Interval::Month => "1M".to_string(),
            Interval::ThreeMonth => "3M".to_string(),
            Interval::SixMonth => "6M".to_string(),
            Interval::Year => "1Y".to_string(),
            Interval::FiveYear => "5Y".to_string(),
        }
    }
}

pub struct State {
    pub assets: HashMap<String, Asset>,
    pub internal_terminal: InternalTerminalState,
    pub interval: Interval,
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
