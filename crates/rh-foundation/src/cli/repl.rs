//! Shared REPL scaffold for interactive read–eval–print loops.
//!
//! Provides [`ReplSession`] to centralise readline setup, history persistence,
//! and Ctrl-C / Ctrl-D handling, eliminating per-crate boilerplate.
//!
//! Enabled via the `repl` crate feature.

use anyhow::Result;
use rustyline::{error::ReadlineError, DefaultEditor};
use std::path::PathBuf;

/// Outcome of a single [`ReplSession::readline`] call.
pub enum ReplInput {
    /// A line of text was successfully read.
    Line(String),
    /// The user pressed Ctrl+C. Typically: clear current input and continue.
    Interrupted,
    /// The user pressed Ctrl+D or closed the input stream. Typically: exit.
    Eof,
}

/// A REPL session wrapping a rustyline editor with optional history persistence.
///
/// History is saved automatically when the session is dropped.
pub struct ReplSession {
    editor: DefaultEditor,
    history_file: Option<PathBuf>,
}

impl ReplSession {
    /// Create a new session without history persistence.
    pub fn new() -> Result<Self> {
        Ok(Self {
            editor: DefaultEditor::new()?,
            history_file: None,
        })
    }

    /// Create a new session that loads history from `history_file` on creation
    /// and saves it when dropped.
    pub fn with_history(history_file: PathBuf) -> Result<Self> {
        let mut editor = DefaultEditor::new()?;
        let _ = editor.load_history(&history_file);
        Ok(Self {
            editor,
            history_file: Some(history_file),
        })
    }

    /// Read one line with the given prompt.
    ///
    /// Translates `ReadlineError::Interrupted` and `ReadlineError::Eof` into
    /// [`ReplInput`] variants so callers never need to import rustyline directly.
    /// All other errors are propagated as `Err`.
    pub fn readline(&mut self, prompt: &str) -> Result<ReplInput> {
        match self.editor.readline(prompt) {
            Ok(line) => Ok(ReplInput::Line(line)),
            Err(ReadlineError::Interrupted) => Ok(ReplInput::Interrupted),
            Err(ReadlineError::Eof) => Ok(ReplInput::Eof),
            Err(err) => Err(err.into()),
        }
    }

    /// Add a line to the readline history buffer.
    pub fn add_history_entry(&mut self, line: &str) -> Result<()> {
        self.editor.add_history_entry(line)?;
        Ok(())
    }
}

impl Drop for ReplSession {
    fn drop(&mut self) {
        if let Some(ref file) = self.history_file {
            let _ = self.editor.save_history(file);
        }
    }
}
