use std::env;
use std::fs;
use std::path::Path;
use std::fs::File;
use std::io::BufRead;

fn example_of_closure() {
    let c = 5;
    let closure = |b| {
        b + c
    };
    closure(1);
}

fn naive_search(target: &str, s: &str) {
    for line in s.lines() {
        if line.contains(target) {
            // new syntax: rust2021?? compiler version 1.60?: println!("{line}");
            println!("{}", line);
        }
    }
}

fn better_search(target: &str, path: impl AsRef<Path>) -> std::io::Result<()> {
    let file = File::open(path)?;
    let mut reader = std::io::BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(line) => {
                if line.contains(target) {
                    println!("{line}");
                }
            },
            Err(e) => eprintln!("{e}")
        }
       
    }
    
    /* what the above would desugar into ^^^
    let mut buffer = String::new();
    while let Ok(_) = reader.read_line(&mut buffer) {

        buffer.clear();
    }
    */

    Ok(())
}

fn better_search_2<P: AsRef<Path>>(target: &str, path: P) {

}

fn better_search_3<P>(target: &str, path: P) 
where
    P: AsRef<Path>
{

}

fn main() {
    let args: Vec<String> = env::args().collect();
    let target = match args.get(1) {
        Some(s) => s,
        None => {
            eprintln!("You didn't give me a target string!");
            std::process::exit(1);
        }
    };
    println!("{target}");
    /*
    let files = match args.get(2..) {
        Some(files) => files,
        None => {
            eprintln!("You didn't give any files!");
            std::process::exit(1);
        }
    };
    */
    let files = args.get(2..).unwrap_or_default();
    println!("{:?}", files);

    for file in files {
        /* 
        let contents = fs::read_to_string(file);
        match contents {
            Ok(contents) => naive_search(target, &contents),
            Err(e) => eprintln!("{e}"),
        }
        */
        better_search(target, file); // not handling an error here!
    }
    /*
    equivalent:
    files.iter().for_each(|f| {

    });
    */
    // let target = &args[1];
    // let files = &args[2..];
    
}
