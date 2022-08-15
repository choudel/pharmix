#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use crate::schemaql::QueryRoot;
use schemaql::Schema;
use tauri_plugin_graphql::Context as GraphQLContext;
use juniper::{EmptySubscription, EmptyMutation};
pub mod schemaql;


fn main() {
    let schema = Schema::new(
        QueryRoot,
        EmptyMutation::<GraphQLContext>::new(),
        EmptySubscription::<GraphQLContext>::new(),
    );
    tauri::Builder::default()
        .plugin(tauri_plugin_graphql::init(schema))
        .run(tauri::generate_context!())
        .expect("failed to run app");
}
