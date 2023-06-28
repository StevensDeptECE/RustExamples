/*
fn readu64() {
  let mut input = String::new();
    io::stdin().read_line(&mut input);

    let number: u64 = input.trim().parse().unwrap_or_else();
  number
}
*/

fn is_prime(n: u64) -> bool {
    let sqrt_n  = (n as f64).sqrt() as u64;
    if n == 2 {
      return true;
    }
    if n % 2 == 0 {
      return false;
    }
    for i in (3..=sqrt_n).step_by(2) {
//        println!(" dividing by {}", i);
        if n % i == 0 {
          return false;
        }
    }

    return true;
}

fn eratosthenes(n : u64) -> u64 {
  let mut bits = bitvec![0; n]; 
  for i in (3..n).step_by(2) {
    bits.set(i / 2, true);   
  }
  let mut count = 1;
  let sqrt_n_2 = (n as f64).sqrt() as u64;
  for i in 1..sqrt_n_2 {
    if (bits[i]) {
        count = count + 1;
        // since i is half the prime number, p^2 is (2*i)^2 = 4*i*i+1
        // then the corresponding bit is is 2*i*i
        for j in (2*i*i..n).step_by(4*i+1) {
          bits.clear(j);
        }
    }
  }
  count
}


fn main() {
  for i in 2..100 {
    if is_prime(i) { print!("{} ", i);}
  }
  println!(); 
  let n = 1000000;
  println!(eratosthenes(n));
}
