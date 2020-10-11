use std::fs::File;
use std::collections::HashMap;
use std::io::{BufReader, BufRead};
use std::env;
fn loadDict(filename : &String) -> HashMap<String,i32> {
    let mut words = HashMap::new();
    let file = File::open(filename).unwrap();
    let r = BufReader::new(file);
    let mut wordCount = 0;
    for line in r.lines() {
	wordCount += 1;
	words.insert(line.unwrap(), wordCount);
    }
    return words;
}

fn main() {
    let args : Vec<String> = env::args().collect(); // Vec<String>
    let words = loadDict(&args[1]); // dictionary is the first arg
    //note here words is not mutable, because it never changes once returned
    
    for i in 2..args.len() { // subsequent args are file to spellcheck 	
	println!("{}", args[i]);
	let r = BufReader::new(File::open(&args[i]).unwrap());
	let mut lineCount = 0;
	for line in r.lines() {
	    lineCount += 1;
	    for word in line.unwrap().split_whitespace() {
		if !words.contains_key(word) {
		    println!("{}: {}", lineCount, word);
		}
	    }
	}
    }
}
