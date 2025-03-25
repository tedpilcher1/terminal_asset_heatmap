use ratatui::{
    layout::Alignment,
    text::{Line, Span},
    widgets::{Paragraph, StatefulWidget, Widget, Wrap},
};

const MAX_CHAR: usize = 20;

// TODO: Save & current stocks to/from some storage
// Add some defaults
pub enum Command {
    Exit,
    AddAsset(String),
    RemoveAsset(String),
    ChangeInterval(String),
}

pub struct InternalTerminalState {
    terminal_buffer: Vec<char>,
}

impl Default for InternalTerminalState {
    fn default() -> Self {
        Self::new()
    }
}

impl InternalTerminalState {
    pub fn new() -> Self {
        Self {
            terminal_buffer: vec![],
        }
    }

    pub fn get_text(&self) -> String {
        self.terminal_buffer.iter().collect()
    }

    pub fn remove_char(&mut self) {
        self.terminal_buffer.pop();
    }

    pub fn add_new_char(&mut self, new_char: char) {
        if self.terminal_buffer.len() < MAX_CHAR {
            self.terminal_buffer.push(new_char);
        }
    }

    pub fn enter_command(&mut self) -> Option<Command> {
        let command = self.terminal_buffer.iter().collect::<String>();
        let split_command: Vec<&str> = command.split(" ").collect();
        self.terminal_buffer.clear();
        match split_command.as_slice() {
            ["EXIT"] => Some(Command::Exit),
            ["ADD", subject] => Some(Command::AddAsset(subject.to_string())),
            ["REMOVE", subject] => Some(Command::RemoveAsset(subject.to_string())),
            ["INTERVAL", subject] => Some(Command::ChangeInterval(subject.to_string())),
            _ => None,
        }
    }
}

pub struct InternalTerminalWidget {}

impl StatefulWidget for InternalTerminalWidget {
    type State = InternalTerminalState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        let text = state.get_text();
        let spans = Line::from(vec![Span::raw("> "), Span::raw(text)]);

        let paragraph = Paragraph::new(spans)
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });

        let bordered_area = ratatui::layout::Rect {
            x: area.x + 2,
            y: area.y + 2,
            width: area.width - 2,
            height: area.height - 4,
        };

        ratatui::widgets::Block::default()
            .borders(ratatui::widgets::Borders::ALL)
            .render(area, buf);

        paragraph.render(bordered_area, buf);
    }
}
