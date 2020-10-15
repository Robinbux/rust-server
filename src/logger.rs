use chrono::prelude::*;
use serde::Serialize;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use tinytemplate;
use tinytemplate::TinyTemplate;
use std::env;

#[derive(Serialize)]
struct Log {
    logs: String,
}

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

    pub fn replace_template_values(html_str: &str) -> String {
        let logs = fs::read_to_string("resources/logs/Log.txt")
            .expect("Something went wrong reading the file");

        let log = Log { logs };

        let mut tt = TinyTemplate::new();
        tt.add_template("log_template", html_str).expect("Unable to add template");

        tt.render("log_template", &log)
            .expect("Unable to render template.")
    }
}
