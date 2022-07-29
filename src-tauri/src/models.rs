use diesel::prelude::*;
use super::schema::posts;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Debug, Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
    
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostJson {
    pub title: String,
    pub body: String,
}

