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
