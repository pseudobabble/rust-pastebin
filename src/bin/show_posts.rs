extern crate rust_pastebin;
extern crate diesel;

use self::rust_pastebin::*;
use self::models::*;
use self::diesel::prelude::*;


fn main() {
    use rust_pastebin::schema::posts::dsl::*;

    let connection = establish_connection();
    let results = posts.filter(published.eq(true))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} published posts", results.len());
    for post in results {
        println!("{} - {}", post.id, post.title);
        println!("----------\n");
        println!("{}", post.body);
    }
}
