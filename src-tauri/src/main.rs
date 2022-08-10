#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use juniper::{
    graphql_object, EmptyMutation, EmptySubscription, FieldResult, GraphQLObject, RootNode,
};
use tauri_plugin_graphql::Context as GraphQLContext;

extern crate dotenv;
extern crate app;
extern crate diesel;
use dotenv::dotenv;
use std::env;
use self::diesel::prelude::*;
use self::app::*;
use self::models::*;



pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
#[derive(GraphQLObject, Debug, Clone)]
struct Post1 {
    id: i32,
    text: String,
    title: String,
    body: String,
    
}

impl Post1 {
    pub fn new(text: String, title: String, body: String) -> Self {
        Self {
            id: rand::random::<i32>(),
            text,
            title,
            body,
            
        }
    }
}

struct Query;

#[graphql_object(context = GraphQLContext)]
impl Query {
    fn list() -> FieldResult<Vec<Post1>> {
        
        use app::schema::posts::dsl::*;
    let connection= establish_connection();
    let results = posts.filter(published.eq(true))
    .limit(5)
    .load::<Post>(&connection)
    .expect("Error loading posts");
        let item = vec![
            Post1::new("foo".to_string(),results[0].title.to_owned(),results[0].body.to_owned()),
            Post1::new("bar".to_string(),results[1].title.to_owned(),results[1].body.to_owned())
            ];
        Ok(item)
    }
}

type Schema =
    RootNode<'static, Query, EmptyMutation<GraphQLContext>, EmptySubscription<GraphQLContext>>;

fn main() {
    
    let schemaql = Schema::new(
        Query,
        EmptyMutation::<GraphQLContext>::new(),
        EmptySubscription::<GraphQLContext>::new(),
    );
    tauri::Builder::default()
        .plugin(tauri_plugin_graphql::init(schemaql))
        .run(tauri::generate_context!())
        .expect("failed to run app");
}
