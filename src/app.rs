use std::io::Write;
use crate::pyt::{self, ShellHandle};

#[derive(Debug, Clone, Copy, PartialEq, Eq)] // its a macro that allows me to debug (print it using {:?}, clone (.clone()), PartialRq (compare using == and !=), copy and set default
pub enum Focus {
    Terminal,
    Agent
}

pub struct App{
    focus : Focus,
    should_quit : bool,
    fullscreen : bool,
    shell : ShellHandle,
    input_buffer : String,
    output_lines : Vec<String>,
    cursor_pos : usize,
    scroll_offset : usize,
}

impl App {
    pub fn new() -> App{
        App{
            focus : Focus::Terminal, //explicitely selects the Terminal from the enum
            should_quit : false,
            fullscreen : false,
            shell : pyt::spawn_shell().expect("Failed to spawn the shell!"),
            input_buffer : String::new(),
            output_lines : Vec::new(),
            cursor_pos : 0,
            scroll_offset : 0,
        }
    }

    pub fn focus_right(&mut self){
        self.focus = Focus::Agent;
    }

    pub fn focus_left(&mut self){
        self.focus = Focus::Terminal;
    }

    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn focus(&self) -> Focus{
        self.focus
    }

    pub fn toggle_fullscreen(&mut self){
        self.fullscreen = !self.fullscreen;
    }

    pub fn fullscreen(&self) -> bool{
        self.fullscreen
    }

    pub fn push_char(&mut self, c: char) {
        let byte_idx = self.input_buffer
            .char_indices()
            .nth(self.cursor_pos)
            .map(|(i, _)| i)
            .unwrap_or(self.input_buffer.len());

        self.input_buffer.insert(byte_idx, c);
        self.cursor_pos += 1;
    }

    pub fn backspace(&mut self) {
        if self.cursor_pos == 0 {
            return;
        }

        let byte_idx = self.input_buffer
            .char_indices()
            .nth(self.cursor_pos - 1)
            .map(|(i, _)| i)
            .unwrap_or(self.input_buffer.len());

        self.input_buffer.remove(byte_idx);
        self.cursor_pos -= 1;
    }

    pub fn input_buffer(&self) -> &str{
        &self.input_buffer
    }

    pub fn output_lines(&self) -> &Vec<String>{
        &self.output_lines
    }

    pub fn submit_input(&mut self) {
        let line = std::mem::take(&mut self.input_buffer);
        self.cursor_pos = 0;

        self.output_lines.push(format!("> {}", line));

        let _ = self.shell.writer.write_all(line.as_bytes());
        let _ = self.shell.writer.write_all(b"\n");
        
        self.reset_scroll();
    }

    pub fn drain_shell_output(&mut self) {
    while let Ok(chunk) = self.shell.output_rx.try_recv() {
        let text = String::from_utf8_lossy(&chunk);
        let clean = strip_ansi_codes(&text);
        for line in clean.lines() {
            self.output_lines.push(line.to_string());
        }
    }
}

    pub fn move_cursor_left(&mut self) {
        if self.cursor_pos > 0 {
            self.cursor_pos -= 1;
        }
    }

    pub fn move_cursor_right(&mut self) {
        let char_count = self.input_buffer.chars().count();
        if self.cursor_pos < char_count {
            self.cursor_pos += 1;
        }
    }

    pub fn cursor_pos(&self) -> usize {
        self.cursor_pos
    }

    pub fn scroll_up(&mut self){
        self.scroll_offset = self.scroll_offset + 1;
    }

    pub fn scroll_down(&mut self){
        
        if self.scroll_offset > 0{
            self.scroll_offset = self.scroll_offset - 1;
        }
    }

    pub fn scroll_offset(&self) -> usize {
        self.scroll_offset
    }

    pub fn reset_scroll(&mut self){
        self.scroll_offset = 0;
    }
    
}

fn strip_ansi_codes(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\x1b' {
            // ESC starts an escape sequence. The common case we're
            // seeing (CSI sequences like `[01;34m`) is ESC followed
            // by '[', then a run of parameter/intermediate bytes,
            // ending in a single "final" byte in the 0x40-0x7E range
            // (letters mostly - 'm' for color, 'h'/'l' for mode
            // toggles like the [?2004h we saw, etc).
            if chars.peek() == Some(&'[') {
                chars.next(); // consume '['
                while let Some(&next) = chars.peek() {
                    chars.next();
                    if next.is_ascii_alphabetic() {
                        break; // this was the final byte, sequence is over
                    }
                }
            }
            // If ESC wasn't followed by '[', we just drop the lone
            // ESC and continue - simplest safe fallback for today.
        } else {
            output.push(c);
        }
    }

    output
}