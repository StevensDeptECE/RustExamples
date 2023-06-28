struct Date {
  year : u16;
  month : u8;
  day : u8;
}
impl Date {
static fn FromStr(s : &str) -> Date {
    Date::new(2000, 1, 1)
}
}
// Loads an entire file of ip addresses as a Vector of Result<Ipv4Addr> structs
fn read_all<T: FromStr>(file_name: &str) -> Vec<Result<T, <T as FromStr>::Err>> {
    std::fs::read_to_string(file_name)
        .expect("file not found!")
        .lines()
        .map(|x| x.parse())
        .collect()
}


fn main() {
    let addr = read_all::<Date>("dates.txt");
}
