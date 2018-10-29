use rocket;

#[get("/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

pub fn create_routes() {
    rocket::ignite().mount("/hello", routes![hello]).launch();
}

