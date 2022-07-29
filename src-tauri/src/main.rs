#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use juniper::{
  graphql_object, graphql_subscription, EmptyMutation, FieldResult, GraphQLObject,
  RootNode, FieldError,
  futures::Stream
};
use tauri_plugin_graphql::Context as GraphQLContext;
use std::pin::Pin;

#[derive(GraphQLObject, Debug, Clone)]
struct Human {
  name: String,
}

struct Query;

#[graphql_object(context = GraphQLContext)]
impl Query {
  fn hero() -> FieldResult<Human> {
    Ok(Human {
      name: "Luke Skywalker".to_string(),
    })
  }
}

pub struct Subscription;

type StringStream = Pin<Box<dyn Stream<Item = Result<String, FieldError>> + Send>>;

#[graphql_subscription(context = GraphQLContext)]
impl Subscription {
    async fn hello_world() -> StringStream {
        let stream = juniper::futures::stream::iter(vec![
            Ok(String::from("Hello")),
            Ok(String::from("World!"))
        ]);

        Box::pin(stream)
    }
}

type Schema = RootNode<
  'static,
  Query,
  EmptyMutation<GraphQLContext>,
  Subscription,
>;

struct Database;

#[derive(serde::Serialize)]
struct CustomResponse{
  message:String,
  other_val:usize,
}
async fn some_function()-> Option<String>{
  Some("playa".into())
}

#[tauri::command]
async fn my_custom_command(
  window: tauri::Window,
  number: usize,
_database: tauri::State<'_,Database>,
)->Result<CustomResponse,String>{
  println!("Called from {}",window.label());
  let result: Option<String>= some_function().await;
  if let Some(message)=result{
    Ok(CustomResponse {
      message,
      other_val: 42+ number,
    })
  }else{
    Err("No result".into())
  }
}

fn main() {
  let schema = Schema::new(
    Query,
    EmptyMutation::<GraphQLContext>::new(),
    Subscription,
  );
  tauri::Builder::default()
    .manage(Database{})
    .invoke_handler(tauri::generate_handler![my_custom_command])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
