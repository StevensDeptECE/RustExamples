// global definition of error codes
enum ErrorCodes { // all error codes should be associated with file, line#
  out_of_mem,     // parameter: size that failed
  file_not_found, // parameter: string filename
  read_prohibited, // parameter: file handle?
  write_prohibited, // parameter: file handle?
  init_failure,
  cleanup_failure,
  permission_denised
}

fn batcherrors(lang : string) {
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => panic!("no such file"),
    };
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .ok()
        .expect("failed to read!");
    let lines: Vec<String> = file_contents.split("\n")
        .map(|s: &str| s.to_string())
        .collect();
    lines
}
