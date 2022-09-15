#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]


extern crate diesel;
extern crate dotenv;

use dotenv::dotenv;
use std::env;
use diesel::{Connection, SqliteConnection};
#[macro_use] 
extern crate diesel_migrations;
embed_migrations!("./migrations/");

use crate::schemaql::Query;
use schemaql::Schema;
use tauri_plugin_graphql::Context as GraphQLContext;
use juniper::{EmptySubscription, EmptyMutation};
pub mod schemaql;


fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn=SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
        diesel_migrations::run_pending_migrations(&conn).expect("Error migrating");
    
        let schema = 
    Schema::new(
        Query,
        EmptyMutation::<GraphQLContext>::new(),
        EmptySubscription::<GraphQLContext>::new(),
    );
    tauri::Builder::default()
        .plugin(tauri_plugin_graphql::init(schema))  
        .run(tauri::generate_context!())
        .expect("failed to run app");
}
