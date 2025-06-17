use lazy_static::lazy_static;
use log::error;
use std::{
    fs::OpenOptions,
    io::Write,
    sync::mpsc::{Sender, channel},
    thread::JoinHandle,
};
use termio::libs::util::timestamp::Timestamp;

const LOG_PATH: &str = "termdot_error.log";

lazy_static! {
    static ref GB_LOB: LocalLog = LocalLog::new(LOG_PATH);
}

pub struct LocalLog {
    _join_handle: JoinHandle<()>,
    sender: Sender<String>,
}

impl LocalLog {
    pub fn new(path: &str) -> Self {
        let (sender, receiver) = channel::<String>();
        let path = path.to_string();

        let _join_handle = std::thread::Builder::new()
            .name("local-log".to_string())
            .spawn(move || {
                let mut file = match OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open(path)
                    .inspect_err(|e| error!("Crate `LocalLog` failed, e = {:?}", e))
                {
                    Ok(f) => f,
                    Err(_) => return,
                };

                loop {
                    if let Ok(mut log) = receiver.recv() {
                        log.insert_str(0, &format!("[{}] ", Timestamp::now().format_string(None)));

                        let _ = file.write_all(log.as_bytes());
                    }
                }
            })
            .expect("[LocalLog::new] Spawn log thread failed.");

        Self {
            _join_handle,
            sender,
        }
    }

    #[inline]
    pub fn append(log: String) {
        if let Err(e) = GB_LOB.sender.send(log) {
            error!("[LocalLog::append] Send log failed, e = {:?}", e)
        }
    }
}

#[macro_export]
macro_rules! gb_info {
    ($($arg:tt)*) => {{
        let mut log = format!($($arg)*);
        log.insert_str(0, "[INFO] ");
        log.push_str("\r\n");
        common::log::LocalLog::append(log);
    }}
}

#[macro_export]
macro_rules! gb_warn {
    ($($arg:tt)*) => {{
        let mut log = format!($($arg)*);
        log.insert_str(0, "[WARN] ");
        log.push_str("\r\n");
        common::log::LocalLog::append(log);
    }}
}

#[macro_export]
macro_rules! gb_error {
    ($($arg:tt)*) => {{
        let mut log = format!($($arg)*);
        log.insert_str(0, "[ERROR] ");
        log.push_str("\r\n");
        common::log::LocalLog::append(log);
    }}
}
