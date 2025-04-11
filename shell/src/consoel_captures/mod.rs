use common::log::LocalLog;
use gag::BufferRedirect;
use std::io::Read;

pub struct ConsoleCaptures {
    stdout: BufferRedirect,
    stderr: BufferRedirect,
}

impl Default for ConsoleCaptures {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl ConsoleCaptures {
    pub fn new() -> Self {
        Self {
            stdout: BufferRedirect::stdout().unwrap(),
            stderr: BufferRedirect::stderr().unwrap(),
        }
    }

    #[inline]
    pub fn read_stdout(&mut self) -> String {
        let mut res = String::new();
        let _ = self.stdout.read_to_string(&mut res).unwrap_or_else(|e| {
            LocalLog::append(format!(
                "[ConsoleCaptures::read_stdout] Read `stdout` failed, e = {:?}",
                e,
            ));
            0
        });
        res
    }

    #[inline]
    pub fn read_stderr(&mut self) -> String {
        let mut res = String::new();
        let _ = self.stderr.read_to_string(&mut res).unwrap_or_else(|e| {
            LocalLog::append(format!(
                "[ConsoleCaptures::read_stdout] Read `stdout` failed, e = {:?}",
                e,
            ));
            0
        });
        res
    }
}
