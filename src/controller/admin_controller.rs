use crate::controller::base_controller::BaseController;
use crate::controller::controller::Controller;
use crate::enums::content_type::ContentType;
use crate::enums::http_status_codes::HTTPStatusCodes;
use crate::server::request::Request;
use crate::server::response::Response;
use crate::services::error_service::ErrorService;
use crate::utils::file_handler::file_handler;
use crate::utils::logger::Logger;

use serde::Serialize;
use std::fs;
use tinytemplate;
use tinytemplate::TinyTemplate;

pub struct AdminController {
    #[allow(dead_code)]
    logger: Logger,
    error_service: ErrorService,
}

mod files {
    pub const CONSOLE: &str = "console.html";
}

impl AdminController {
    pub fn new() -> AdminController {
        let logger = Logger::new(String::from("AdminController"));
        let error_service = ErrorService::new();
        AdminController {
            logger,
            error_service,
        }
    }

    fn console(&self) -> Response {
        let data_vec =
            &file_handler::load_resource(files::CONSOLE).expect("Unable to load resource");
        let content = AdminController::replace_template_values(
            &file_handler::convert_vec_to_string(data_vec),
        );
        Response::new(content, ContentType::HTML, HTTPStatusCodes::Ok)
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
    fn execute_request(&mut self, request: &mut Request) -> Response {
        request.current_child_path = BaseController::extract_child_path(&request.resource_path);
        let route_beginning = BaseController::extract_parent_path(&request.current_child_path);
        return match route_beginning {
            "console" => self.console(),
            _ => self.error_service.serve_404_page(),
        };
    }
}

#[derive(Serialize)]
struct Log {
    logs: String,
}
