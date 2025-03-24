pub mod assets;
pub mod internal_terminal;
pub mod state;

use anyhow::Result;
use assets::Asset;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use internal_terminal::{Command, InternalTerminalWidget};
use ratatui::Terminal;
use ratatui::layout::Offset;
use ratatui::prelude::Backend;
use ratatui::prelude::Constraint::{Fill, Length, Min};
use ratatui::text::Text;
use ratatui::widgets::{Block, StatefulWidget};
use ratatui::{Frame, layout::Layout};
use state::{Interval, State};
use yahoo_finance_api::YahooConnector;

fn main() -> Result<()> {
    let mut terminal = ratatui::init();
    let mut state = State::new();

    loop {
        draw(&mut terminal, &mut state);
        if let Some(command) = handle_events(&mut state)? {
            match command {
                Command::Exit => break,
                Command::AddAsset(ticker) => {
                    let asset = Asset::new(ticker.clone());
                    state.assets.insert(ticker, asset);
                }
                Command::RemoveAsset(ticker) => {
                    state.assets.remove_entry(&ticker);
                }
                Command::ChangeInterval(interval) => {
                    if let Ok(interval) = Interval::try_from(interval) {
                        state.interval = interval
                    }
                }
            }
        }
    }
    ratatui::restore();

    Ok(())
}

fn draw(terminal: &mut Terminal<impl Backend>, state: &mut State) {
    let internal_terminal_text = Text::raw(state.internal_terminal.get_text());
    let stocks_text = Text::raw(
        state
            .assets
            .keys()
            .map(|k| k.as_str())
            .collect::<Vec<&str>>()
            .join(", "),
    );
    let interval_text = Text::raw(String::from(state.interval.clone()));

    let vertical = Layout::vertical([Length(1), Min(0), Length(5), Length(5)]);

    let _ = terminal.draw(|frame| {
        let [title_area, heatmap_area, interval_area, terminal_area] = vertical.areas(frame.area());
        // frame.render_widget(internal_terminal_text, frame.area());
        // frame.render_widget(stocks_text, frame.area());
        // frame.render_widget(interval_text, frame.area());
        frame.render_widget(Block::bordered().title("Asset Heatmap"), title_area);
        let internal_terminal = &mut state.internal_terminal;
        frame.render_stateful_widget(InternalTerminalWidget {}, terminal_area, internal_terminal);
    });
}

fn handle_events(state: &mut State) -> Result<Option<Command>> {
    let command = match event::read()? {
        Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
            KeyCode::Char(char) => {
                state.internal_terminal.add_new_char(char);
                None
            }
            KeyCode::Backspace => {
                state.internal_terminal.remove_char();
                None
            }
            KeyCode::Enter => state.internal_terminal.enter_command(),
            _ => None,
        },
        _ => None,
    };

    Ok(command)
}
