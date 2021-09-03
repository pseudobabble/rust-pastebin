extern crate rust_pastebin;
extern crate diesel;

use self::rust_pastebin::*;
use self::models::*;
use self::diesel::prelude::*;

use crate::diesel::associations::HasTable;

fn main() {
    use rust_pastebin::schema::posts::dsl::*;

    let connection = establish_connection();
    let results = posts
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("----------\n");
        println!("{}", post.body);
    }
}