fn hello() {
  println!("hello");
}

fn f(x:f64) ->f64 {
  x*x
}

fn g(x:f64) ->f64 {
  x*x - 2.0
}

fn gp(x:f64) ->f64 {
  2.0*x
}

fn hypot(x : f64, y : f64) -> f64 {
  return (x*x + y*y).sqrt();
}

//fn countPrime(n : u64) -> u64 {
//  let mut count = 1_u64; // special case for 2
//  let mut i = 3;
//  while i < n {
//    let mut j = 3;
//    while j <= sqrt(i) {
//      if i % j == 0      
//}

fn bisect(f: fn(f64) -> f64, mut a : f64, mut b : f64, eps : f64) -> f64 {
  loop { 
    let x = (a+b)/2.0;
    let y = f(x);
    if y < 0.0 {
      a = x;
    } else if y > 0.0 {
      b = x;
    } else {
      return x;
    }
    if (b-a).abs() < eps {
      return x;
    }
  }  
}

/*
   Compute root using Newton-Raphson method, requiring function f and
   derivative f'
*/
fn newton_raphson(f: fn(f64) -> f64, fp: fn(f64) -> f64, mut x0 : f64, eps : f64) -> f64 {
  loop {
    let x1 = x0 - f(x0) / fp(x0);
    if (x1-x0).abs() < eps {
      return x1;
    }
    x0 = x1;
  }
}

fn main() {
  hello();
  println!("{}", f(3.0));
  println!("{}", hypot(3.0,4.0));
  println!("{}", hypot(4.0,5.0));
  println!("{}", bisect(g, 0.0, 2.0, 0.0001));
  println!("{}", newton_raphson(g, gp, 2.0, 0.0001));
  println!("{}", newton_raphson(g, gp, 2.0, 1.0e-10));
}
