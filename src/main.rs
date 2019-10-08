#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod authentication;

mod api;
use api::inventory::item;

fn main() {
    rocket::ignite()
        .mount("/item", routes![item::get_item])
        .launch();
}
