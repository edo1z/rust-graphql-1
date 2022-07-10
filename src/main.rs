use async_graphql::*;

struct Query;

#[Object]
impl Query {
    async fn add(&self, a: i32, b: i32) -> MyObject {
        MyObject { a, b }
    }
}

#[derive(SimpleObject)]
struct MyObject {
    a: i32,
    b: i32,
}

#[tokio::main]
async fn main() {
    let schema = Schema::new(Query, EmptyMutation, EmptySubscription);
    println!("{}", &schema.sdl());
    let res = schema.execute("{add(a: 10, b:20) {a}}").await;
    let res_json = serde_json::to_string(&res);
    println!("{}", res_json.unwrap());
}
