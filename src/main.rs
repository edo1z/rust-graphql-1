use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema};

struct Query;

#[Object]
impl Query {
    async fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }
}

#[tokio::main]
async fn main() {
    let schema = Schema::new(Query, EmptyMutation, EmptySubscription);
    let res = schema.execute("{add(a: 10, b:20)}").await;
    let res_json = serde_json::to_string(&res);
    println!("{}", res_json.unwrap());
}
