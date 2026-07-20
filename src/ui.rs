use crate::app::{App, Focus};
use crate::theme;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::Style;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

/// Entry point called from main.rs's terminal.draw() closure.
pub fn render(frame: &mut Frame, app: &App) {
    let background = Block::default().style(Style::default().bg(theme::BACKGROUND));
    frame.render_widget(background, frame.area());

    let outer = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(frame.area());

    render_main_panels(frame, app, outer[0]);
    render_input_bar(frame, app, outer[1]);
}

/// The outer "Parakeet v1.0.0" frame. This is app-level chrome, not
/// a focus target itself, so it always renders at full brightness -
/// only the two panels INSIDE it independently react to focus.
fn render_main_panels(frame: &mut Frame, app: &App, area: Rect) {
    let outer_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme::GREEN))
        .title(" Parakeet v1.0.0 ");

    let inner_area = outer_block.inner(area);
    frame.render_widget(outer_block, area);

    let panels = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(inner_area);

    render_output_panel(frame, app, panels[0]);
    render_agent_panel(frame, app, panels[1]);
}

/// Each panel independently owns a full 4-sided box (Borders::ALL,
/// always) - no shared divider, no dependency on the other panel's
/// state. Only the COLOR reacts to focus: full brand green when
/// focused (a real, visible box), BORDER_INACTIVE when not (still a
/// real border in the code, but close enough to BACKGROUND to read
/// as invisible). This is what makes both panels behave identically
/// instead of one "growing into" a box and the other not.
fn render_output_panel(frame: &mut Frame, app: &App, area: Rect) {
    let is_focused = app.focus() == Focus::Terminal;
    let text_style = theme::text_style_for(is_focused);
    let border_style = theme::border_style_for(is_focused);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(border_style)
        .title(" Output Panel ")
        .title_style(text_style);

    frame.render_widget(Paragraph::new("").style(text_style).block(block), area);
}

fn render_agent_panel(frame: &mut Frame, app: &App, area: Rect) {
    let is_focused = app.focus() == Focus::Agent;
    let text_style = theme::text_style_for(is_focused);
    let border_style = theme::border_style_for(is_focused);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(border_style)
        .title(" Agent Panel ")
        .title_style(text_style);

    frame.render_widget(Paragraph::new("").style(text_style).block(block), area);
}

/// The 3-line command input box with the brand's prompt glyph.
fn render_input_bar(frame: &mut Frame, _app: &App, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme::GREEN));

    let prompt_text = format!("{} ", theme::PROMPT_GLYPH);
    let paragraph = Paragraph::new(prompt_text)
        .style(Style::default().fg(theme::GREEN))
        .block(block);

    frame.render_widget(paragraph, area);
}