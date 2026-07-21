mod app;
mod ui;
mod theme;
mod pyt;
mod banner;

use app::App;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::ExecutableCommand;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io::{self, Stdout, Write};

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

/// Shows the wordmark splash on the REAL terminal (normal scrollback,
/// no raw mode, no alt screen) and waits for the user to press Enter
/// before we ever touch the terminal state. Must run before
/// init_terminal(): once we're in the alternate screen this output
/// would be invisible and the user's shell history would never see it.
fn show_splash() -> io::Result<()> {
    banner::print_banner();
    print!("Press Enter to continue...");
    io::stdout().flush()?;

    // Block on a real line read here (not crossterm::event::read) -
    // we're deliberately still in normal/cooked terminal mode, so a
    // plain line-buffered read is the right tool, not raw key events.
    let mut discard = String::new();
    io::stdin().read_line(&mut discard)?;
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

    show_splash()?;

    let mut terminal = init_terminal()?;
    let mut app = App::new();

    let result = run_app(&mut terminal, &mut app);

    // Normal (non-panic) exit path also restores the terminal.
    restore_terminal()?;

    result
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    app: &mut App,
) -> io::Result<()> {
    loop {
        terminal.draw(|frame| ui::render(frame, app))?;

        // Wait up to 16ms for a key event. If nothing arrives in
        // that window, poll() returns false and we fall through -
        // we do NOT block indefinitely here anymore, because we
        // also need to check the shell's channel every tick, not
        // just when a key happens to be pressed.
        if event::poll(std::time::Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
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
                    (KeyCode::Char('t'), KeyModifiers::CONTROL) => {
                        app.toggle_fullscreen();
                    }
                    (KeyCode::Enter, _) => {
                        app.submit_input();
                    }
                    (KeyCode::Backspace, _) => {
                        app.backspace();
                    }
                    (KeyCode::Char('h'), KeyModifiers::CONTROL) => {
                        app.backspace();
                    }
                    (KeyCode::Left, _) => {
                        app.move_cursor_left();
                    }
                    (KeyCode::Right, _) => {
                        app.move_cursor_right();
                    }
                    (KeyCode::Char(c), _) => {
                        app.push_char(c);
                    }
                    (KeyCode::Up, _) => {
                        app.scroll_up();
                    }
                    (KeyCode::Down, _) => {
                        app.scroll_down();
                    }
                    _ => {}
                }
                }
            }
        }

        // Runs every single loop tick, key or no key - this is what
        // makes shell output show up even while you're not typing.
        app.drain_shell_output();

        if app.should_quit() {
            break;
        }
    }
    Ok(())
}