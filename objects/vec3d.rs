use std::fmt;

#[derive(Debug)]
struct Vec3d {
    x: f64, y: f64, z: f64
}

impl fmt::Display for Vec3d {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
	write!(f,"({},{},{})", self.x, self.y, self.z)
    }
}

impl Vec3d {
    fn add(&self, b:&Vec3d) -> Vec3d {
	Vec3d{x: self.x + b.x, y: self.y + b.y, z: self.z + b.z}
    }
    fn add2(a:&Vec3d , b:&Vec3d) -> Vec3d {
	Vec3d{x: a.x + b.x, y: a.y + b.y, z: a.z + b.z}
    }

    fn sub(&self, b:&Vec3d) -> Vec3d {
	Vec3d{x: self.x - b.x, y: self.y - b.y, z: self.z - b.z}
    }
    fn dot(&self, b:&Vec3d) -> f64 {
	self.x*b.x + self.y*b.y + self.z*b.z
    }
}

fn main() {
    let v1 = Vec3d{x:1.0, y:2.0, z:5.2};
    let v2 = Vec3d{x:2.5, y:3.5, z:1.5};

    let a = v1.add(&v2);
    println!("{:?} + {:?} = {:?}", v1, v2, a);
    println!("{} + {} = {}", v1, v2, Vec3d::add2(&v1, &v2));
    println!("{} + {} = {}", v1, v2, v1.sub(&v2));
    println!("{} dot {} = {}", v1, v2, v1.dot(&v2));
    println!("{}", a);
}
