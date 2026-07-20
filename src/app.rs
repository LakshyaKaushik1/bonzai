use portable_pty::{CommandBuilder, PtySize, native_pty_system};
use anyhow::Error;


#[derive(Debug, Clone, Copy, PartialEq, Eq)] // its a macro that allows me to debug (print it using {:?}, clone (.clone()), PartialRq (compare using == and !=), copy and set default
pub enum Focus {
    Terminal,
    Agent
}

pub struct App{
    focus : Focus,
    should_quit : bool,
}

impl App {
    pub fn new() -> App{
        App{
            focus : Focus::Terminal, //explicitely selects the Terminal from the enum
            should_quit : false,
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

    
}

