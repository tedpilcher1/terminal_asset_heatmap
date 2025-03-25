pub mod assets;
pub mod internal_terminal;
pub mod interval;
pub mod state;

use anyhow::Result;
use assets::Asset;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use internal_terminal::{Command, InternalTerminalWidget};
use interval::{Interval, IntervalWidget};
use ratatui::Terminal;
use ratatui::layout::{Layout, Margin};
use ratatui::prelude::Backend;
use ratatui::prelude::Constraint::{Length, Min};
use ratatui::widgets::Block;
use state::{AssetHeatmapWidget, State};
use yahoo_finance_api::YahooConnector;

#[tokio::main]
async fn main() -> Result<()> {
    let mut finance_provider =
        YahooConnector::new().expect("Should be able to create YahooConnector");
    let mut terminal = ratatui::init();
    let mut state = State::new_with_assets(&mut finance_provider).await;

    loop {
        draw(&mut terminal, &mut state);
        if let Some(command) = handle_events(&mut state)? {
            match command {
                Command::Exit => break,
                Command::Update => {
                    state
                        .update_interval(state.interval.clone(), &mut finance_provider)
                        .await
                }
                Command::AddAsset(ticker) => {
                    if let Ok(asset) =
                        Asset::try_new(ticker.clone(), &mut finance_provider, &state.interval).await
                    {
                        state.assets.insert(ticker, asset);
                    }
                }
                Command::RemoveAsset(ticker) => {
                    state.assets.remove_entry(&ticker);
                }
                Command::ChangeInterval(interval) => {
                    if let Ok(interval) = Interval::try_from(interval) {
                        state.update_interval(interval, &mut finance_provider).await
                    }
                }
            }
        }
    }
    ratatui::restore();

    Ok(())
}

fn draw(terminal: &mut Terminal<impl Backend>, state: &mut State) {
    let vertical = Layout::vertical([Length(1), Min(0), Length(3), Length(3)]);
    let _ = terminal.draw(|frame| {
        let [title_area, heatmap_area, interval_area, terminal_area] = vertical.areas(frame.area());
        frame.render_widget(Block::bordered().title(" Asset Heatmap "), title_area);
        let internal_terminal = &mut state.internal_terminal;
        frame.render_stateful_widget(InternalTerminalWidget {}, terminal_area, internal_terminal);
        frame.render_stateful_widget(IntervalWidget {}, interval_area, &mut state.interval);
        frame.render_stateful_widget(
            AssetHeatmapWidget {},
            heatmap_area.inner(Margin::new(1, 1)),
            &mut state.assets,
        );
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
