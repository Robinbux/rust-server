use crate::controller::base_controller::BaseController;
use crate::controller::controller::Controller;
use crate::enums::content_type::ContentType;
use crate::utils::file_handler::file_handler;
use crate::utils::logger::Logger;
use crate::server::request::Request;
use serde::Serialize;
use std::fs;
use tinytemplate;
use tinytemplate::TinyTemplate;

pub struct AdminController {
    #[allow(dead_code)]
    logger: Logger,
}

mod files {
    pub const CONSOLE: &str = "console.html";
}

impl AdminController {
    pub fn new() -> AdminController {
        let logger = Logger::new(String::from("AdminController"));
        AdminController { logger: logger }
    }

    fn console(&self) -> Vec<u8> {
        let data_vec =
            &file_handler::load_resource(files::CONSOLE).expect("Unable to load resource");
        AdminController::replace_template_values(&file_handler::convert_vec_to_string(data_vec))
    }

    fn replace_template_values(html_str: &str) -> Vec<u8> {
        let logs = fs::read_to_string("resources/logs/Log.txt")
            .expect("Something went wrong reading the file");

        let log = Log { logs };

        let mut tt = TinyTemplate::new();
        tt.add_template("log_template", html_str)
            .expect("Unable to add template");

        let template_render = tt.render("log_template", &log).unwrap();
        let template_render: &[u8] = template_render.as_ref();
        let template_render_vec = template_render.to_vec();
        template_render_vec
    }
}

impl Controller for AdminController {
    fn execute_request(&self, request: Request) -> Result<Vec<u8>, Vec<u8>> {
        let route_beginning = BaseController::extract_parent_path(&request.resource_path);
        return match route_beginning {
            "console" => Ok(self.console()),
            _ => Err(BaseController::serve_404_page()),
        };
    }

    fn get_content_type_for_path(&self, path: &str) -> ContentType {
        let route_beginning = BaseController::extract_parent_path(&path);
        return match route_beginning {
            "console" => ContentType::HTML,
            _ => ContentType::HTML,
        };
    }
}

#[derive(Serialize)]
struct Log {
    logs: String,
}
