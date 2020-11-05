pub mod db_handler {
    use postgres::{Client, NoTls};

    pub fn setup_database()  {
        let client = Client::connect("host=localhost user=postgres", NoTls);
        client.unwrap().simple_query(r#"
        CREATE TABLE IF NOT EXISTS notes (
            id              int   PRIMARY KEY,
            note_message    text
        );
        "#);
    }
}