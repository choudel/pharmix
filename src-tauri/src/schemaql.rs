extern crate diesel;
use app::{*, models::Post};
use diesel::prelude::*;
use juniper::{graphql_object, EmptySubscription, EmptyMutation, FieldResult, GraphQLObject, RootNode};
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
        use schema::posts::dsl::*;
        let connection = establish_connection();
        let results = posts
            .filter(published.eq(true))
            .limit(1)
            .load::<Post>(&connection)
            .expect("Error loading posts");
        Self {
            id:results[0].id,
            title:results[0].title.to_string(),
            body:results[0].body.to_string(),
            published:results[0].published,
        }
    }
}

pub struct QueryRoot;

#[graphql_object(context = GraphQLContext)]
impl QueryRoot {
    async fn list() -> FieldResult<ListItem> {
        use schema::posts::dsl::*;
        let connection = establish_connection();
        let results = posts
            .filter(published.eq(true))
            .limit(1)
            .load::<Post>(&connection)
            .expect("Error loading posts");
        Ok(ListItem{
            id:results[0].id.to_owned(),
            title:results[0].title.to_string().to_owned(),
            body:results[0].body.to_string().to_owned(),
            published:results[0].published.to_owned(),

        })
    
    }
}


pub(crate) type Schema = RootNode<'static, QueryRoot, EmptyMutation<GraphQLContext>, EmptySubscription<GraphQLContext>>;
