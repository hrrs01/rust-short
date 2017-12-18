

use std::fs::File;
use std::io::prelude::*;
use std::env;
use std::io::stdin;

enum TokenType {
    INTEGER(u32),
    PLUS(String),
    EOF(u32)
}

struct Interpereter {

    code: String,
    len: u32,

}

impl Interpereter {

    fn read_file(&self, filename: &str) -> String {
        let mut file = File::open(&filename).expect("File not found");

        let mut content = String::new();

        file.read_to_string(&mut content).expect("Could not read file");

        content
    }

    fn get_args(&self) -> String {
        let args: Vec<_> = env::args().collect();
        if args.len() > 1 {

            println!("{}", &args[1]);
        }
        String::from(args[1].trim())
    }

    fn exec(&self) {
        let filename = self.get_args();

        println!("Opening file {}", &filename);

        println!("{}", self.read_file(&filename));

    }

}
fn main() {



}
