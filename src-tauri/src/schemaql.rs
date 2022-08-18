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
    pub fn new(id:i32) -> Self {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let conn = SqliteConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
        let result =models::Post::show(id, &conn);
        Self {
            id,
            title: result[0].title.to_string().to_owned(),
            body: result[0].body.to_string().to_owned(),
            published: result[0].published.to_owned()
        }
    }
}
    
pub struct Query;

#[graphql_object(context = GraphQLContext)]
impl Query {
    async fn list() -> FieldResult<Vec<ListItem>> {
        let item = vec![
            ListItem::new(2),
            ListItem::new(3)
        ]; 
        Ok(item)
    }
}

pub(crate) type Schema =
    RootNode<'static, Query, EmptyMutation<GraphQLContext>, EmptySubscription<GraphQLContext>>;
