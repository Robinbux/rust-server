use chrono::prelude::*;
use std::env;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;

pub struct Logger {
    name: String,
    file: File,
}

impl Logger {
    pub fn new(name: String) -> Logger {
        let key = "TESTING";
        let log_path = match env::var_os(key) {
            Some(_val) => String::from("resources/logs/test_log.txt"),
            None => String::from("resources/logs/Log.txt"),
        };

        let file = OpenOptions::new()
            .append(true)
            .open(&log_path)
            .expect("cannot open file");
        Logger { name, file }
    }

    pub fn log(&mut self, log_message: &str) {
        let now: DateTime<Local> = Local::now();
        let complete_message = format!("{:?}  [{}]: {}\n", now, self.name, log_message);
        self.file
            .write_all(complete_message.as_bytes())
            .expect("write failed")
    }
}
