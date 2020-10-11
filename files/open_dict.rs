use std::fs::File;
use std::collections::HashMap;
use std::io::{self, BufReader, BufRead};
fn main() {
    let mut words = HashMap::new();
    let file = File::open("../Greek.dic").unwrap();
    let r = BufReader::new(file);
    let mut wordCount = 0;
    for line in r.lines() {
	wordCount += 1;
	words.insert(line.unwrap(), wordCount);
//	println!("{}", line.unwrap());
    }
    if words.contains_key("αγκαλιαστά") {
	println!("{}", words["αγκαλιαστά"]);
    }

    // this form only asks once. Does the optimizer fix the above?
    if let Some(count) = words.get("αγκαλιαστά") {
	println!("{}", count);
    }    
}
