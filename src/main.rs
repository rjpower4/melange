use std::fs::File;
use std::io::{BufReader, Read};

use crate::daf::FileRecord;

mod byteorder;
mod daf;

fn main() {
    let mut input = BufReader::new(File::open("de440.bsp").expect("Could not open"));
    let mut buffer = vec![0; 1024];

    input.read_exact(buffer.as_mut_slice()).expect("shit, man");

    let fr = FileRecord::try_from(buffer.as_slice()).expect("failed try from");

    println!("{:?}", String::from_utf8_lossy(&fr.description));
}
