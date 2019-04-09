//! This module is used for enabling and disabling raw mode for the terminal.
//!
//! What exactly is raw state:
//! - No line buffering.
//!    Normally the terminals uses line buffering. This means that the input will be send to the terminal line by line.
//!    With raw mode the input will be send one byte at a time.
//! - Input
//!   All input has to be written manually by the programmer.
//! - Characters
//!   The characters are not processed by the terminal driver, but are sent straight through.
//!   Special character have no meaning, like backspace will not be interpret as backspace but instead will be directly send to the terminal.
//! - Escape characters
//!   Note that in raw modes `\n` `\r` will move to the new line but the cursor will be at the same position as before on the new line therefor use `\n\r` to start at the new line at the first cell.
//!
//! With these modes you can easier design the terminal screen.

use crate::sys;
use std::io::{self, Stdout, Write};

/// A wrapper for the raw terminal state. Which can be used to write to.
///
/// Although this type is available for you to use I would recommend using `Screen` instead.
/// Note that when you want to use input and raw mode you should use `Screen`.
pub struct RawScreen;

impl RawScreen {
    /// Put terminal in raw mode.
    pub fn into_raw_mode() -> io::Result<RawScreen> {
        #[cfg(unix)]
        let mut command = sys::unix::RawModeCommand::new();
        #[cfg(windows)]
        let mut command = sys::winapi::RawModeCommand::new();

        let _result = command.enable();

        Ok(RawScreen)
    }

    /// Put terminal back in original modes.
    pub fn disable_raw_mode() -> io::Result<()> {
        #[cfg(unix)]
        let mut command = sys::unix::RawModeCommand::new();
        #[cfg(windows)]
        let command = sys::winapi::RawModeCommand::new();

        command.disable()?;
        Ok(())
    }
}

/// Types which can be converted into "raw mode".
///
/// # Why is this type defined on writers and not readers?
///
/// TTYs has their state controlled by the writer, not the reader. You use the writer to clear the
/// screen, move the cursor and so on, so naturally you use the writer to change the mode as well.
pub trait IntoRawMode: Write + Sized {
    /// Switch to raw mode.
    ///
    /// Raw mode means that stdin won't be printed (it will instead have to be written manually by
    /// the program). Furthermore, the input isn't canonicalised or buffered (that is, you can
    /// read from stdin one byte of a time). The output is neither modified in any way.
    fn into_raw_mode(self) -> io::Result<RawScreen>;
}

impl IntoRawMode for Stdout {
    fn into_raw_mode(self) -> io::Result<RawScreen> {
        RawScreen::into_raw_mode()?;
        // this make's sure that raw screen will be disabled when it goes out of scope.
        Ok(RawScreen {})
    }
}

impl Drop for RawScreen {
    fn drop(&mut self) {
        RawScreen::disable_raw_mode().unwrap();
    }
}
