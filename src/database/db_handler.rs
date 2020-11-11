pub mod db_handler {
    extern crate diesel;
    extern crate dotenv;

    use diesel::prelude::*;
    use diesel::pg::PgConnection;
    use dotenv::dotenv;
    use std::env;

    pub fn establish_connection() -> PgConnection {
        dotenv().ok();
    
        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");
        PgConnection::establish(&database_url)
            .expect("Error connecting to DB")
    }
}