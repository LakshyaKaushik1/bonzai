use std::io;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    DefaultTerminal,
};

struct App {
    current_command:String,
    terminal_log : Vec<String>,
}

impl App{
    fn new() -> App {
        App {
            current_command: String::new(),
            terminal_log : Vec::new()}
    }

    fn type_char(&mut self, c : char){
        self.current_command.push(c);
    }

    fn backspace(&mut self){
            self.current_command.pop();
    }

    fn submit(&mut self){
        self.terminal_log.push(self.current_command.clone());
        self.current_command.clear();
    }
}

fn main() -> io::Result<()> {

    let mut terminal = ratatui::init(); // setup: raw mode + alternate screen, in one call
    let result = run(&mut terminal);
    ratatui::restore(); // teardown: guaranteed to undo setup
    result
}

fn run(terminal: &mut DefaultTerminal) -> io::Result<()> {
    let mut app = App::new();

    loop {
        terminal.draw(|frame| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(1),    // history area: take all remaining space
                    Constraint::Length(3), // input area: exactly 3 lines tall
                ])
                .split(frame.area());

            let history_text = app.terminal_log.join("\n");
            let history = Paragraph::new(history_text)
                .block(Block::default().title("bonzai").borders(Borders::ALL));
            frame.render_widget(history, chunks[0]);

            let input = Paragraph::new(app.current_command.as_str())
                .block(Block::default().title("input").borders(Borders::ALL));
            frame.render_widget(input, chunks[1]);
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Esc => break,
                KeyCode::Backspace => app.backspace(),
                KeyCode::Char(c) => app.type_char(c),
                KeyCode::Enter => app.submit(),
                _ => {}
            }
        }
    }
    Ok(())
}