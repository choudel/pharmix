use diesel;
use diesel::prelude::*;

use crate::schema::posts;
use crate::schema::posts::dsl::posts as all_posts;
use crate::schema::drugs;
use crate::schema::drugs::dsl::drugs as all_drugs;
#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}
impl Post {
    pub fn show(id:i32,conn:&SqliteConnection)->Vec<Post>{
        all_posts
        .find(id)
        .load::<Post>(conn)
        .expect("Error loading book")
    }
    pub fn all(conn:&SqliteConnection)->Vec<Post>{
        all_posts
        .order(posts::id.desc())
        .load::<Post>(conn)
        .expect("Error loading the books")
    }
}
#[derive(Queryable)]
pub struct Drug {
    pub id: i32,
    pub dci: String,
    pub description: String
}
impl Drug {
    pub fn show(id:i32,conn:&SqliteConnection)->Vec<Drug>{
        all_drugs
        .find(id)
        .load::<Drug>(conn)
        .expect("Error loading book")
    }
    pub fn all(conn:&SqliteConnection)->Vec<Drug>{
        all_drugs
        .order(drugs::id.desc())
        .load::<Drug>(conn)
        .expect("Error loading the books")
    }
}
