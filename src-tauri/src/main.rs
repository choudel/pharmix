#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use juniper::{graphql_object, EmptySubscription, EmptyMutation, FieldResult, GraphQLObject, RootNode};
use tauri_plugin_graphql::Context as GraphQLContext;
#[derive(GraphQLObject, Debug, Clone)]
struct Post {
    id: i32,
    text: String,
    
}

impl Post {
    pub fn new(text: String) -> Self {
        Self {
            id: rand::random::<i32>(),
            text,
        }
    }
}

struct Query;

#[graphql_object(context = GraphQLContext)]
impl Query {
    fn list() -> FieldResult<Vec<Post>> {
        let item = vec![
            Post::new("foo".to_string()),
            Post::new("bar".to_string())
        ];
        Ok(item)
    }
}



type Schema = RootNode<'static, Query, EmptyMutation<GraphQLContext>, EmptySubscription<GraphQLContext>>;

fn main() {
  let schema = Schema::new(
    Query,
    EmptyMutation::<GraphQLContext>::new(),
    EmptySubscription::<GraphQLContext>::new(),
);
    tauri::Builder::default()
        .plugin(tauri_plugin_graphql::init(schema))
        .run(tauri::generate_context!())
        .expect("failed to run app");
}
