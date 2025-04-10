use std::io::Read;

use common::log::LocalLog;
use gag::BufferRedirect;

pub struct ConsoleCaptures {
    stdout: BufferRedirect,
    stderr: BufferRedirect,
}

impl ConsoleCaptures {
    #[inline]
    pub fn new() -> Self {
        Self {
            stdout: BufferRedirect::stdout().unwrap(),
            stderr: BufferRedirect::stderr().unwrap(),
        }
    }

    #[inline]
    pub fn read_stdout(&mut self) -> String {
        let mut stdout = String::new();
        let _ = self.stdout.read_to_string(&mut stdout).unwrap_or_else(|e| {
            LocalLog::append(format!(
                "[ConsoleCaptures::read_stdout] Read `stdout` failed, e = {:?}",
                e,
            ));
            0
        });
        stdout
    }

    #[inline]
    pub fn read_stderr(&mut self) -> String {
        let mut stderr = String::new();
        let _ = self.stderr.read_to_string(&mut stderr).unwrap_or_else(|e| {
            LocalLog::append(format!(
                "[ConsoleCaptures::read_stdout] Read `stdout` failed, e = {:?}",
                e,
            ));
            0
        });
        stderr
    }
}
