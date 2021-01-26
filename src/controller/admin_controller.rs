use crate::controller::base_controller::BaseController;
use crate::controller::controller::Controller;
use crate::enums::content_type::ContentType;
use crate::enums::http_status_codes::HTTPStatusCodes;
use crate::server::request::Request;
use crate::server::response::Response;
use crate::services::error_service::ErrorService;
use crate::services::resource_service::ResourceService;
use crate::utils::logger::Logger;

use serde::Serialize;
use std::fs;
use std::str;
use tinytemplate;
use tinytemplate::TinyTemplate;

#[derive(Clone)]
pub struct AdminController {
    #[allow(dead_code)]
    logger: Logger,
    error_service: ErrorService,
    resource_service: ResourceService,
}

mod files {
    pub const CONSOLE: &str = "resources/html/console.html";
}

impl AdminController {
    pub fn new() -> AdminController {
        let logger = Logger::new(String::from("AdminController"));
        let error_service = ErrorService::new();
        let resource_service = ResourceService::new();
        AdminController {
            logger,
            error_service,
            resource_service,
        }
    }

    fn console(&self) -> Response {
        let data_vec = ResourceService::load_from_file_path(files::CONSOLE)
            .expect("Unable to load console file");
        let content = AdminController::replace_template_values(str::from_utf8(&data_vec).unwrap());
        Response::new(content, ContentType::HTML, HTTPStatusCodes::Ok)
    }

    fn replace_template_values(html_str: &str) -> Vec<u8> {
        let logs = fs::read_to_string("resources/logs/Log.txt")
            .expect("Something went wrong reading the log file");

        let log = Log { logs };

        let mut tt = TinyTemplate::new();
        tt.add_template("log_template", html_str)
            .expect("Unable to add template");

        let template_render = tt.render("log_template", &log).unwrap();
        let template_render: &[u8] = template_render.as_ref();
        template_render.to_vec()
    }
}

impl Controller for AdminController {
    fn execute_request(&self, mut request: Request) -> Response {
        request.current_child_path = BaseController::extract_child_path(&request.resource_path);
        let route_beginning = BaseController::extract_parent_path(&request.current_child_path);
        match route_beginning {
            "console" => self.console(),
            _ => self.error_service.serve_404_page(),
        }
    }
}

#[derive(Serialize)]
struct Log {
    logs: String,
}
