use std::process::{Command, Output, Stdio};
use std::path::Path;

/// Safe wrapper for executing system commands with proper error handling
#[derive(Debug)]
pub struct SafeCommand {
    inner: Command,
}

impl SafeCommand {
    /// Create a new SafeCommand with the given program
    pub fn new<P: AsRef<std::ffi::OsStr>>(program: P) -> Self {
        Self {
            inner: Command::new(program),
        }
    }

    /// Add arguments to the command
    pub fn args<I, S>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<std::ffi::OsStr>,
    {
        self.inner.args(args);
        self
    }

    /// Set the working directory for the command
    pub fn current_dir<P: AsRef<Path>>(mut self, dir: P) -> Self {
        self.inner.current_dir(dir);
        self
    }

    /// Execute the command and return the output
    pub fn output(mut self) -> Result<Output, Box<dyn std::error::Error>> {
        // Configure the command for safe execution
        self.inner
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .stdin(Stdio::null());

        let output = self.inner.output()?;

        // Log the command execution for debugging
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            log::debug!("Command failed with stderr: {}", stderr);
        }

        Ok(output)
    }

    /// Execute the command and return only the stdout as a String
    pub fn output_text(self) -> Result<String, Box<dyn std::error::Error>> {
        let output = self.output()?;
        Ok(String::from_utf8(output.stdout)?)
    }

    /// Execute the command and check if it succeeded
    pub fn status(self) -> Result<bool, Box<dyn std::error::Error>> {
        let output = self.output()?;
        Ok(output.status.success())
    }
}