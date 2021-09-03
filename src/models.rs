use serde::{Serialize, Deserialize};
use super::schema::posts;

#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
#[table_name="posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}
