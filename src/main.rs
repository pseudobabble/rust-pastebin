#[macro_use] extern crate rocket;
extern crate rust_pastebin;
extern crate diesel;

use std::iter::Iterator;
use rocket::{Rocket, Build};
use rocket::fairing::AdHoc;
use rocket::response::{Debug, status::Created};
use serde::{Serialize, Deserialize};
use rocket_sync_db_pools::database;
use diesel::prelude::*;


use self::rust_pastebin::*;
use self::models::*;

// #[get("/<name>/<age>")]
// fn get_hello(name: String, age: u8) -> Json<Person> {
//     NOTE: In a real application, we'd use `rocket_contrib::json::Json`.
//     let person = Person { name: name, age: age, };
//     Json(serde_json::to_string(&person).unwrap())
//     Json(person)
// }

// #[post("/<age>", format = "plain", data = "<name_data>")]
// fn post_hello(age: u8, name_data: Data) -> Result<Json<String>, Debug<io::Error>> {
//     let mut name = String::with_capacity(32);
//     name_data.open().take(32).read_to_string(&mut name)?;
//     let person = Person { name: name, age: age, };
//     NOTE: In a real application, we'd use `rocket_contrib::json::Json`.
//     Ok(Json(serde_json::to_string(&person).expect("valid JSON")))
// }
//


#[database("hello_rocket")]
struct Database(diesel::SqliteConnection);

#[get("/")]
fn index(connection: &Database) -> String {
    use rust_pastebin::schema::posts::dsl::*;

//    connection.run(|c| posts.filter(published.eq(true)).limit(5).load(c))
    let results = posts.filter(published.eq(true)).limit(5).load::<Post>(&*connection).expect("BORK");

    // Convert the Point to a JSON string.
    let serialized = serde_json::to_string(&results).unwrap();

//   serialized
   "Success".to_string()

}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Database::fairing())
        .mount("/", routes![index])
}
