use std::io::{Read, Write};
use portable_pty::{native_pty_system, CommandBuilder, PtySize};
use anyhow::Result;

pub struct ShellHandle {
    pub reader: Box<dyn Read + Send>,
    pub writer: Box<dyn Write + Send>,
}

pub fn spawn_shell() -> Result<ShellHandle> {
    let pty_system = native_pty_system();
    let pair = pty_system.openpty(PtySize {
        rows: 24,
        cols: 80,
        pixel_width: 0,
        pixel_height: 0,
    })?;

    // Spawn a shell into the pty
    let cmd = CommandBuilder::new("bash");
    let _child = pair.slave.spawn_command(cmd)?;

    // Read and parse output from the pty with reader
    let clone_reader = pair.master.try_clone_reader()?;

    // Get the writer
    let take_writer = pair.master.take_writer()?;

    // Send data to the pty by writing to the master
    Ok(ShellHandle {
        reader: clone_reader,
        writer: take_writer,
    })
}