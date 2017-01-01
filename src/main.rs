#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rand;

use rand::distributions::{IndependentSample, Range};
use std::fs::File;

static SAMPLE_SPACE: &'static [u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/")]
fn post_index() -> String {
    gen_code()
}

/**
 * Saves a file with the given name
 */
fn touch<T: AsRef<str>>(name: T) {
    File::create(name.as_ref());
}

/**
 * Generates a random unique code. 
 */
fn gen_code() -> String {
    let mut rand = rand::thread_rng();
    let between = Range::new(0, SAMPLE_SPACE.len());
    (0..8).map(|_| SAMPLE_SPACE[between.ind_sample(&mut rand)] as char).collect()
}

fn main() {
    rocket::ignite().mount("/", routes![index, post_index]).launch();
}

