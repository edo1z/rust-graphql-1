use async_graphql::*;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

struct Query;

#[Object]
impl Query {
    async fn books(&self, ctx: &Context<'_>) -> Vec<Book> {
        let storage = ctx.data_unchecked::<Arc<Mutex<Storage>>>().lock().unwrap();
        storage.books.clone()
    }
    async fn authors(&self, ctx: &Context<'_>) -> Vec<Author> {
        let storage = ctx.data_unchecked::<Arc<Mutex<Storage>>>().lock().unwrap();
        storage.authors.clone()
    }
}

struct Mutation;
#[Object]
impl Mutation {
    async fn add_book(&self, ctx: &Context<'_>, book_name: String) -> Book {
        let mut storage = ctx.data_unchecked::<Arc<Mutex<Storage>>>().lock().unwrap();
        let new_book = Book {
            id: Uuid::new_v4().into(),
            name: book_name,
            author: None,
        };
        storage.books.push(new_book.clone());
        new_book
    }
    async fn add_author(&self, ctx: &Context<'_>, author_name: String) -> Author {
        let mut storage = ctx.data_unchecked::<Arc<Mutex<Storage>>>().lock().unwrap();
        let new_author = Author {
            id: Uuid::new_v4().into(),
            name: author_name,
            books: None,
        };
        storage.authors.push(new_author.clone());
        new_author
    }
}

#[derive(SimpleObject, Clone)]
struct Book {
    id: ID,
    name: String,
    author: Option<Author>,
}
#[derive(SimpleObject, Clone)]
struct Author {
    id: ID,
    name: String,
    books: Option<Vec<Book>>,
}

struct Storage {
    books: Vec<Book>,
    authors: Vec<Author>,
}
impl Storage {
    fn new() -> Self {
        Self {
            books: vec![],
            authors: vec![],
        }
    }
}

async fn exec_query(schema: &Schema<Query, Mutation, EmptySubscription>, query_str: &str) {
    let res = schema.execute(query_str).await;
    let res_json = serde_json::to_string(&res);
    println!("{}", res_json.unwrap());
}

fn print_sdl(schema: &Schema<Query, Mutation, EmptySubscription>) {
    println!("----------");
    println!("{}", schema.sdl());
    println!("----------");
}

#[tokio::main]
async fn main() {
    let mut _storage = Storage::new();
    let book = Book {
        id: Uuid::new_v4().into(),
        name: "book1".to_string(),
        author: None,
    };
    _storage.books.push(book);
    let storage = Arc::new(Mutex::new(_storage));

    let schema = Schema::build(Query, Mutation, EmptySubscription)
        .data(storage)
        .finish();
    print_sdl(&schema);
    exec_query(&schema, "query {books { name }}").await;
    let query_str = "mutation {addBook(bookName:\"book2\") { id name }}";
    exec_query(&schema, query_str).await;
    exec_query(&schema, "query {books { id name }}").await;
}
