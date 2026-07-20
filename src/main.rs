mod app;
mod ui;
mod theme;
mod pyt;

use app::App;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::ExecutableCommand;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io::{self, Stdout};

/// Puts the terminal into the state our TUI needs:
/// raw mode (see every keystroke immediately) + alternate screen
/// (draw over a blank buffer instead of the user's scrollback).
fn init_terminal() -> io::Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    io::stdout().execute(EnterAlternateScreen)?;
    let terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
    Ok(terminal)
}

/// Undoes exactly what init_terminal did. This must run no matter how
/// the program exits - normal return OR panic - or the user's real
/// shell is left in a broken state after we quit.
fn restore_terminal() -> io::Result<()> {
    disable_raw_mode()?;
    io::stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn main() -> io::Result<()> {
    // Install a panic hook that restores the terminal BEFORE the
    // default panic handler prints its message. Without this, a
    // panic mid-raw-mode leaves the user's actual terminal broken -
    // they'd see garbled output until they blindly typed `reset`.
    let default_panic_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        // Best-effort: if restoring fails here too, there's nothing
        // more we can do, so we ignore the error rather than panic
        // again inside a panic handler.
        let _ = restore_terminal();
        default_panic_hook(panic_info);
    }));

    let mut terminal = init_terminal()?;
    let mut app = App::new();

    let result = run_app(&mut terminal, &mut app);

    // Normal (non-panic) exit path also restores the terminal.
    restore_terminal()?;

    result
}

/// The main loop: draw the current state, wait for an input event,
/// update state in response, repeat until should_quit is set.
fn run_app(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    app: &mut App,
) -> io::Result<()> {
    loop {
        terminal.draw(|frame| ui::render(frame, app))?;

        if let Event::Key(key) = event::read()? {
            // On some platforms/terminals a single physical keypress
            // can generate both a Press and a Release KeyEvent. We
            // only want to react once per press, not twice.
            if key.kind == KeyEventKind::Press {
                match (key.code, key.modifiers) {
                    (KeyCode::Char('q'), KeyModifiers::CONTROL) => {
                        app.quit();
                    }
                    (KeyCode::Right, KeyModifiers::CONTROL) => {
                        app.focus_right();
                    }
                    (KeyCode::Left, KeyModifiers::CONTROL) => {
                        app.focus_left();
                    }
                    _ => {}
                }
            }
        }

        if app.should_quit() {
            break;
        }
    }
    Ok(())
}