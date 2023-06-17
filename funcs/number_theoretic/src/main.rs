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

fn main() {
  for i in 2..100 {
    if is_prime(i) { print!("{} ", i);}
  }
  println!(); 
}
