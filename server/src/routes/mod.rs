use graphql_schema;
use rocket;
use juniper_rocket;

#[get("/hello/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[get("/graphql")]
fn graphiql() -> rocket::response::content::Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}

#[get("/graphql?<request>")]
fn get_graphql_handler(
    context: rocket::State<graphql_schema::Context>,
    request: juniper_rocket::GraphQLRequest,
    schema: rocket::State<graphql_schema::Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}

#[post("/graphql", data = "<request>")]
fn post_graphql_handler(
    context: rocket::State<graphql_schema::Context>,
    request: juniper_rocket::GraphQLRequest,
    schema: rocket::State<graphql_schema::Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}

