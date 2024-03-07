fn polar2rect(x: f64, y: f64) -> (f64, f64) {
   let r = (x * x + y * y).sqrt();
   let theta = y.atan2(x);
   (r, theta) 
}

fn main() {
    println!("{:?}", polar2rect(3.0, 4.0));
}
