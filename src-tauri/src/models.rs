use diesel;
use diesel::prelude::*;

use crate::schema::posts;
use crate::schema::posts::dsl::posts as all_posts;
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