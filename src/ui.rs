use crate::app::{App, Focus};
use crate::banner;
use crate::theme;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::Style;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;
use ratatui::text::Line;

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

/// The outer "Parakeet v1.0.0" frame, doubling as the app's status
/// bar: mode indicator on the left, title centered, clock on the
/// right - all three living on the border line itself via multiple
/// .title() calls with independent alignment. This is app-level
/// chrome, not a focus target itself, so it always renders at full
/// brightness - only the two panels INSIDE it independently react
/// to focus.
/// When app.fullscreen() is true, skips the 50/50 split and gives
/// the whole inner area to whichever panel is currently focused.
fn render_main_panels(frame: &mut Frame, app: &App, area: Rect) {
    let mode_label = match app.focus() {
        Focus::Terminal => " (Terminal) ",
        Focus::Agent => " (Agent) ",
    };

    let now = chrono::Local::now();
    let clock_label = format!(" ({}) ", now.format("%d %b %H:%M %a"));

    let outer_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme::GREEN))
        .title(Line::from(mode_label).left_aligned())
        .title(Line::from(" Parakeet v1.0.0 ").centered())
        .title(Line::from(clock_label).right_aligned());

    let inner_area = outer_block.inner(area);
    frame.render_widget(outer_block, area);

    if app.fullscreen() {
        match app.focus() {
            Focus::Terminal => render_output_panel(frame, app, inner_area),
            Focus::Agent => render_agent_panel(frame, app, inner_area),
        }
    } else {
        let panels = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(inner_area);

        render_output_panel(frame, app, panels[0]);
        render_agent_panel(frame, app, panels[1]);
    }
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

    let inner_height = area.height.saturating_sub(2) as usize; // minus top+bottom border
    let all_lines = app.output_lines();
    let total = all_lines.len();

    let scroll_offset = app.scroll_offset().min(total);
    let end = total - scroll_offset;
    let start = end.saturating_sub(inner_height);

    let visible_text = all_lines[start..end].join("\n");

    frame.render_widget(Paragraph::new(visible_text).style(text_style).block(block), area);
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

    // Same text_style as the border/title: GREEN when this panel is
    // focused, GREEN_DIM when it isn't - the banner dims/brightens
    // in lockstep with the rest of the panel automatically.
    frame.render_widget(
        Paragraph::new(banner::AGENT_BANNER).style(text_style).block(block),
        area,
    );
}

/// The 3-line command input box with the brand's prompt glyph.
fn render_input_bar(frame: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme::GREEN));

    let prompt_prefix = format!("{} ", theme::PROMPT_GLYPH);
    let prompt_text = format!("{}{}", prompt_prefix, app.input_buffer());
    let paragraph = Paragraph::new(prompt_text)
        .style(Style::default().fg(theme::GREEN))
        .block(block);

    frame.render_widget(paragraph, area);

    let prefix_width = prompt_prefix.chars().count() as u16;
    let cursor_x = area.x + 1 + prefix_width + app.cursor_pos() as u16;
    let cursor_y = area.y + 1;
    frame.set_cursor_position((cursor_x, cursor_y));
}