use ratatui::{
    style::{Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Paragraph, StatefulWidget, Widget},
};

const MAX_CHAR: usize = 20;

pub enum Command {
    Exit,
    Update,
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
            ["UPDATE"] => Some(Command::Update),
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
        let spans = Line::from(vec![
            Span::raw(" > "),
            Span::raw(text),
            Span::styled("█", Style::default().fg(ratatui::style::Color::DarkGray)),
        ]);

        let input = Paragraph::new(spans)
            .style(Style::default())
            .block(Block::bordered().title(" Terminal "))
            .add_modifier(Modifier::RAPID_BLINK);

        input.render(area, buf);
    }
}
