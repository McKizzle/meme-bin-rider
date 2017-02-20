#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rand;

use rand::distributions::{IndependentSample, Range};
use std::fs::{File, DirBuilder, metadata};
use std::io::Write;
use std::str;
use rocket::Data;

static SAMPLE_SPACE: &'static [u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
static MEME_DIR: &'static str = "meme-bin";

impl AsRef< for Data {
    fn as_ref(&self) -> &Data {
        return &self;
    }
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/", format = "text/plain", data = "<data>")]
fn post_index(data: Data) -> String {
    let name = gen_code() + ".meme";
    
    touch(name.as_str());
    //write_image(name.as_str(),  data);
    
    name
}

fn write_image<T: AsRef<str>, D: AsRef<str>>(name: T, data: D) {
    let path = MEME_DIR.to_owned() + "/" + name.as_ref();
    match File::create(path.as_str()) {
        Ok(mut f) => {
            match f.write(data.as_ref().open().bytes()) {
                Ok(usz) => println!("Wrote {} bytes to {}", usz, path),
                Err(e)  => println!("Cannot write to {}: {}", e, path),
            }
            f.flush();
        },
        Err(e) => println!("Failed to open {} due to: {}", name.as_ref(), e),
    }
}

// fn upload_data(data: Data) -> io::Result<Plain<String>> {
//     
//     let mut f = try!(File::open(
// }

/**
 * Saves a file with the given name
 */
fn touch<T: AsRef<str>>(name: T) {
    match metadata(MEME_DIR) {
        Ok(m) => {
            match m.is_dir() {
                _ => (),
                false => { 
                    match DirBuilder::new().create(MEME_DIR) {
                        Ok(()) => println!("Created the directory {}..", MEME_DIR),
                        Err(e) => println!("Failed to create the directory {}", e),
                    }
                }
            }
        },
        Err(e) => println!("Unable to fetch the meta-data for meme-bin {}", e),
    }

    match File::create(MEME_DIR.to_owned() + "/" + name.as_ref()) {
        Ok(f) => (),
        Err(e) => println!("Failed to create {} due to: {}", name.as_ref(), e),
    }
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

