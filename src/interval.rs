use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Padding, Paragraph, StatefulWidget, Widget},
};

#[derive(Clone)]
pub enum Interval {
    Day,
    Week,
    Month,
    ThreeMonth,
    SixMonth,
    Year,
    TwoYear,
    FiveYear,
    YearToDay,
}

impl Interval {
    pub fn to_provider_format(&self) -> &str {
        match self {
            Interval::Day => "1d",
            Interval::Week => "1wk",
            Interval::Month => "1mo",
            Interval::ThreeMonth => "3mo",
            Interval::SixMonth => "6mo",
            Interval::Year => "1y",
            Interval::TwoYear => "2y",
            Interval::FiveYear => "5y",
            Interval::YearToDay => "ytd",
        }
    }
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
            "2Y" => Ok(Self::TwoYear),
            "5Y" => Ok(Self::FiveYear),
            "YTD" => Ok(Self::YearToDay),
            _ => Err(()),
        }
    }
}

impl From<&Interval> for String {
    fn from(interval: &Interval) -> Self {
        match interval {
            Interval::Day => "1D".to_string(),
            Interval::Week => "1W".to_string(),
            Interval::Month => "1M".to_string(),
            Interval::ThreeMonth => "3M".to_string(),
            Interval::SixMonth => "6M".to_string(),
            Interval::Year => "1Y".to_string(),
            Interval::TwoYear => "2Y".to_string(),
            Interval::FiveYear => "5Y".to_string(),
            Interval::YearToDay => "YTD".to_string(),
        }
    }
}

pub struct IntervalWidget {}

impl StatefulWidget for IntervalWidget {
    type State = Interval;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        let selected_interval = String::from(&*state);
        let intervals = [
            ("1D", Interval::Day),
            ("1W", Interval::Week),
            ("1M", Interval::Month),
            ("3M", Interval::ThreeMonth),
            ("6M", Interval::SixMonth),
            ("1Y", Interval::Year),
            ("2Y", Interval::TwoYear),
            ("5Y", Interval::FiveYear),
            ("YTD", Interval::YearToDay),
        ];

        let mut spans = Vec::with_capacity(intervals.len() * 2 - 1);

        for (i, (label, _)) in intervals.iter().enumerate() {
            spans.push(Span::styled(
                *label,
                if selected_interval == *label {
                    Style::default().fg(Color::Blue)
                } else {
                    Style::default()
                },
            ));

            if i < intervals.len() - 1 {
                spans.push(Span::raw("  |  "));
            }
        }

        let spans = Line::from(spans);
        let input = Paragraph::new(spans).style(Style::default()).block(
            Block::default()
                .padding(Padding::new(2, 0, 0, 0))
                .borders(ratatui::widgets::Borders::TOP),
        );

        input.render(area, buf);
    }
}
