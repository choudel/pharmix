extern crate diesel;
extern crate dotenv;

use app::models;
use diesel::{Connection, SqliteConnection};
use dotenv::dotenv;
use std::env;

use juniper::{
    graphql_object, EmptyMutation, EmptySubscription, FieldResult, GraphQLObject, RootNode,
};
use tauri_plugin_graphql::Context as GraphQLContext;

#[derive(GraphQLObject, Debug, Clone)]
struct ListItem {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

impl ListItem {
    pub fn new() -> Self {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let conn = SqliteConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
        let results = models::Post::all(&conn);
        Self {
            id: results[0].id,
            title: results[0].title.to_string(),
            body: results[0].body.to_string(),
            published: results[0].published,
        }
    }
}

pub struct QueryRoot;

#[graphql_object(context = GraphQLContext)]
impl QueryRoot {
    async fn list() -> FieldResult<ListItem> {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let conn = SqliteConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
        let results = models::Post::all(&conn);
        Ok(ListItem {
            id: results[0].id.to_owned(),
            title: results[0].title.to_string().to_owned(),
            body: results[0].body.to_string().to_owned(),
            published: results[0].published.to_owned(),
        })
    }
}

pub(crate) type Schema =
    RootNode<'static, QueryRoot, EmptyMutation<GraphQLContext>, EmptySubscription<GraphQLContext>>;
