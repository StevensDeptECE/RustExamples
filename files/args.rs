use std::env;
fn main() {
    for arg in env::args().skip(1) { // first arg is name of the program
	println!("{}", arg);
    }
}
