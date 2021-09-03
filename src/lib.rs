#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;
use self::models::{Post, NewPost};

pub mod schema;
pub mod models;


pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}


pub fn create_post<'a>(conn: &SqliteConnection, title: &'a str, body: &'a str) -> Post {
    use schema::posts::dsl;
    use schema::posts;

    let new_post = NewPost {
        title: title,
        body: body,
    };

    diesel::insert_into(dsl::posts)
        .values(&new_post)
        .execute(conn)
        .expect("Error saving new post");


    // .filter(published.eq(true))
    let result = dsl::posts
        .limit(1)
        .load::<Post>(conn)
        .expect("Error loading posts");

    result[0].clone()
}
