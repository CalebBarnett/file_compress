extern crate flate2;

use flate2::write::GzEncoder;
use  flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;
use std::time::Instant;


fn main() {
    //Check arguments being passed. 1) program name 2) source file name 3) target file name
    if args().len() !=3 {
        eprintln!("Usage: 'source' 'target'");
        return;
    }
    //input is first argument passed in
    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
    //output is second argument passed in
    let output = File::create(args().nth(2).unwrap()).unwrap();
    //compress data
    let mut encoder = GzEncoder::new(output, Compression::default());
    //get start time
    let start = Instant::now();
    //copy data from reader to writer
    copy(&mut input, &mut encoder).unwrap();
    //finalizes the compressed data
    let output = encoder.finish().unwrap();
    //length of input data
    println!(
        "Source len: {:?}",
        input.get_ref().metadata().unwrap().len()
     );
     //length of compressed data
     println!("Target len: {:?}", output.metadata().unwrap().len());
     //time from start to now
     println!("Elapsed: {:?}", start.elapsed());
}