// std::env::args() gives an iterator of the given arguments. The first entry at index 0 will be
// the name your prohram was called, the ones that follow are what the user wrote afterwards. 
//
// let pattern = std::env::args().nth(1).expect("no pattern given");
// let path = std::env::args().nth(2).expect("no path given");
// 
// CLI arguments as a data type
//
// struct Cli {
//      pattern: String, 
//      path: std::path::PathBuf,
// }
//
// This defines a new structure (a struct) that has to fields to store data in: pattern and path
//
// Clap: The most popular library for parsing command-line arguments. 

#![allow(unused)]

use clap::Parser;

// Search for a pattern in a file and display the lines that contain it. 
#[derive(Parser)]
struct Cli {
    // The pattern to look for
    pattern: String, 
    // The path to the file to read
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    println!("{}", args);
}
