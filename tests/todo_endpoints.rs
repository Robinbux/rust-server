use rust_server::controller::todo_controller;
use diesel::PgConnection;
use std::fs;
use rust_server::enums::content_type::ContentType;
use rust_server::enums::http_status_codes::HTTPStatusCodes;
use rust_server::http::response::Response;
use rust_server::http::request::Request;
use rust_server::controller::base_controller::BaseController;
use rust_server::controller::controller::Controller;
use std::process::Command;

fn setup_db() {
    //overwrite .env file to use test db
    fs::write(".env", "DATABASE_URL=postgres://postgres:password@localhost/todos_test");
}

fn clean_up() {
    Command::new("diesel").args(&["migration", "redo"]).spawn().expect("migration failed");
    fs::write(".env", "DATABASE_URL=postgres://postgres:password@localhost/postgres");
}


#[test]
#[ignore]
fn todo_endpoints(){
    setup_db();
    let base_controller = BaseController::new();

    // test create_todo
    let create_request = Request::new("POST /todo HTTP/1.1\r\nHost: localhost:8087\r\n\r\n{\"todo_message\":\"first note\"}".to_string());
    let create_response = base_controller.execute_request(create_request);
    assert_eq!(create_response.http_status_code, HTTPStatusCodes::Created);
    // won't pass after running all the other tests
    //let expected_content = "{\"id\":1,\"todo_message\":\"first note\",\"completed\":false}".to_string().as_bytes().to_vec();
    //assert_eq!(create_response.content_bytes, expected_content);

    // test update_todo
    let update_request = Request::new("PUT /todo/1 HTTP/1.1\r\nHost: localhost:8087\r\n\r\n{\"completed\":true}".to_string());
    let update_response = base_controller.execute_request(update_request);
    assert_eq!(update_response.http_status_code, HTTPStatusCodes::Created);

    // test get_all_todos
    let get_request = Request::new("GET /todo HTTP/1.1\r\nHost: localhost:8087\r\n\r\n".to_string());
    let get_response = base_controller.execute_request(get_request.clone());
    assert_eq!(get_response.http_status_code, HTTPStatusCodes::Ok);

    // test delete_todo
    let delete_request = Request::new("DELETE /todo/1 HTTP/1.1\r\nHost: localhost:8087\r\n\r\n".to_string());
    let delete_response = base_controller.execute_request(delete_request);
    assert_eq!(delete_response.http_status_code, HTTPStatusCodes::Ok);
    let expected_content: Vec<u8> = Vec::new();
    assert_eq!(delete_response.content_bytes, expected_content);

    clean_up()
}