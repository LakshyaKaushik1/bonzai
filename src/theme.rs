//! Central source of truth for Parakeet's visual identity, pulled
//! directly from brandkit.html's documented hex values. Nothing in
//! ui.rs should ever write a raw Color::Rgb(...) literal - if a new
//! color is needed, it gets named and added here first, so the whole
//! palette stays auditable against the brand kit in one place.

use ratatui::style::{Color, Style};

/// Panel/app background. Brand kit: "on charcoal" contrast baseline
/// used throughout the semantic color table.
pub const BACKGROUND: Color = Color::Rgb(0x1d, 0x1f, 0x21);

// pub const BACKGROUND: Color = Color::Rgb(0x27, 0x29, 0x31);

// pub const BACKGROUND: Color = Color::Rgb(0x22, 0x24, 0x2a);

/// "Parakeet Green" - brand kit's primary color, body plumage.

pub const GREEN: Color = Color::Rgb(0x5F, 0xB8, 0x5C);
// pub const GREEN: Color = Color::Rgb(0x5F, 0xB8, 0x5C);

/// Brand kit's own darker green (--pk-green-d). Used for unfocused
/// panel TEXT (titles, content) - confirmed working, dimmed but
/// still clearly readable. Do not reuse this for borders; see
/// BORDER_INACTIVE below, which is a deliberately different color
/// for a deliberately different job (near-invisible, not just dim).
pub const GREEN_DIM: Color = Color::Rgb(0x3F, 0x8A, 0x3D);

/// Brand kit's dedicated "Borders / dividers" color (documented in
/// its semantic table against this exact background, #22301f). Used
/// only for unfocused panel borders: present in the code, close
/// enough to BACKGROUND to read as "no border" at a glance.
pub const BORDER_INACTIVE: Color = Color::Rgb(0x33, 0x38, 0x30);

/// Primary readable text (13.8:1 contrast per brand kit).
pub const FG: Color = Color::Rgb(0xE9, 0xE6, 0xDC);

/// The prompt glyph. Brand kit uses this specific heavy angle-quote
/// throughout its mockup, not a plain ASCII '>'.
pub const PROMPT_GLYPH: &str = "❯";

/// Text style (titles, content) for a panel: bright when focused,
/// GREEN_DIM (readable-but-subdued) when not.
pub fn text_style_for(is_focused: bool) -> Style {
    if is_focused {
        Style::default().fg(GREEN)
    } else {
        Style::default().fg(GREEN_DIM)
    }
}

/// Border style for a panel: bright when focused, BORDER_INACTIVE
/// (near-invisible against BACKGROUND) when not. Deliberately
/// separate from text_style_for - borders and text dim to different
/// degrees, so they must never share one helper or one color.
pub fn border_style_for(is_focused: bool) -> Style {
    if is_focused {
        Style::default().fg(GREEN)
    } else {
        Style::default().fg(BORDER_INACTIVE)
    }
}