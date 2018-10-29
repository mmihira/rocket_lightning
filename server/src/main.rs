#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate rocket;
mod routes;

fn main() {
    routes::create_routes();
}
