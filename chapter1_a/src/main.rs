use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_path = Path::new(&args[1]);
    let input_display = input_path.display();

    let mut file = match File::open(input_path) {
        Err(why) => panic!("couldn't open {}: {}", input_display, why.description()),
        Ok(file) => file,
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Err(why) => panic!("couldn't read {}: {}", input_display, why.description()),
        Ok(_) => print!("{}", contents),
    };

    let output_path = Path::new(&args[2]);
    let output_display = output_path.display();

    let mut file = match File::create(output_path) {
        Err(why) => panic!("couldn't create {}: {}", output_display, why.description()),
        Ok(file) => file,
    };

    match file.write_all(contents.as_bytes()) {
        Err(why) => panic!("couldn't write {}: {}", output_display, why.description()),
        Ok(_) => {},
    }
}
