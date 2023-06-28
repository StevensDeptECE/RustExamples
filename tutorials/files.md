# Loading files in rust

1.loading the entire file in a String.

This is done using the std::fs::read_to_string() method. If you're familiar with Python or Ruby, this method is as convenient as Python's read() function or Ruby's File.read() methods. Combined with the power of generics, you can easily build a vector of structs, matching the data type in a file:

```rust
// Loads an entire file of ip addresses as a Vector of Result<Ipv4Addr> structs
fn read_all<T: FromStr>(file_name: &str) -> Vec<Result<T, <T as FromStr>::Err>> {
    std::fs::read_to_string(file_name)
        .expect("file not found!")
        .lines()
        .map(|x| x.parse())
        .collect()
}

let addr = read_all::<Ipv4Addr>("ipv4.txt");
```

2.using the lines() iterator.

This is another easy method for reading a file line by line, using the lines() iterator. This iterator operates on a BufReader created from a File object. So a BufReader structure needs to be created for this to be used.

This example function calls a closure on each line:

```rust
// Calls *func()* on each line
fn read_iter(file_name: &str, func: fn(&str)) {
    let file = File::open(file_name).expect("file not found!");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        func(&line.unwrap());
    }
}
```

This method is useful for small files but not really appropriate for very large files because each iteration incurs a String::new() allocation.

3.using the read_line() function.

The read_line() function can make use of the same String buffer, without reallocating on each iteration. But due to the way Rust iterators work, we can't build a standard iterator here. We have to use a mere loop construct, and stop it when the read_line() function returns Ok(0), which means EOF:

// Reuse the same String buffer
fn read_line(file_name: &str, func: fn(&str)) -> Result<(), std::io::Error> {
    // open target file
    let file = File::open(&file_name)?;

    // uses a reader buffer
    let mut reader = BufReader::new(file);
    let mut line = String::new();

    loop {
        match reader.read_line(&mut line) {
            Ok(bytes_read) => {
                // EOF: save last file address to restart from this address for next run
                if bytes_read == 0 {
                    break;
                }

                func(&line);

                // do not accumulate data
                line.clear();
            }
            Err(err) => {
                return Err(err);
            }
        };
    }

    Ok(())
}

Don't forget to clear the buffer after you've got the data, otherwise buffer will grow unexpectedly.

One can also use the split() iterator, which incurs the same drawback than lines(): it allocates a
Vec<u8> on each iteration.

// Reuse the same Vec<u8> buffer
fn read_split(file_name: &str, func: fn(&[u8])) -> Result<(), std::io::Error> {
    // open target file
    let file = File::open(&file_name)?;

    // uses a reader buffer
    let reader = BufReader::new(file);

    // use a for loop construct
    for line in reader.split(0x10) {
        func(&line?);
    }
    Ok(())
}

4.use mmap()api

For an explantion of this system call, have a look to https://en.wikipedia.org/wiki/Mmap.
As it's not included in the standard Rust library, you can use the memmap crate:

```rust
// Maps the file to memory
fn read_mmap(file_name: &str) -> Result<(), std::io::Error> {
    // open target file
    let file = File::open(&file_name)?;

    // create a memmap struct. After that, mmap variable maps directly file contents
    let mmap = unsafe { Mmap::map(&file)? };

    // use from_utf8() to convert to an UTF8 Rust string
    for s in mmap.split(|x| *x == 0x10) {
        println!("{:?}", std::str::from_utf8(&s).unwrap());
    };

    Ok(())
}
```

Beware this is not a foolproof process, as if the file is changed, you can could get a SIGBUS error.


