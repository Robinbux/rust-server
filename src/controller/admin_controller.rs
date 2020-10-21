use crate::controller::base_controller::BaseController;
use crate::controller::controller::Controller;
use crate::enums::content_type::ContentType;
use crate::utils::file_handler::file_handler;
use crate::utils::logger::Logger;
use serde::Serialize;
use std::fs;
use tinytemplate;
use tinytemplate::TinyTemplate;

pub struct AdminController {
    #[allow(dead_code)]
    logger: Logger,
}

impl Controller for AdminController {
    fn serve_content(&self, path: &str) -> String {
        let route_beginning = BaseController::extract_parent_path(&path);
        return match route_beginning {
            "console" => self.console(),
            _ => panic!("Unknown Path"),
        };
    }

    fn get_content_type_for_path(&self, path: &str) -> ContentType {
        let route_beginning = BaseController::extract_parent_path(&path);
        return match route_beginning {
            "console" => ContentType::HTML,
            _ => panic!("Unknown Path"),
        };
    }
}

mod files {
    pub const CONSOLE: &str = "console.html";
}

impl AdminController {
    pub fn new() -> AdminController {
        let logger = Logger::new(String::from("AdminController"));
        AdminController { logger: logger }
    }

    fn console(&self) -> String {
        AdminController::replace_template_values(
            &file_handler::load_resource(files::CONSOLE).expect("Unable to load resource")[..],
        )
    }

    fn replace_template_values(html_str: &str) -> String {
        let logs = fs::read_to_string("resources/logs/Log.txt")
            .expect("Something went wrong reading the file");

        let log = Log { logs };

        let mut tt = TinyTemplate::new();
        tt.add_template("log_template", html_str)
            .expect("Unable to add template");

        tt.render("log_template", &log)
            .expect("Unable to render template.")
    }
}

#[derive(Serialize)]
struct Log {
    logs: String,
}
