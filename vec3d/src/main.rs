
#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
struct Vec3d {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3d {
    fn add(&self, other: &Vec3d) -> Vec3d {
        Vec3d {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
    fn sub(&self, other: &Vec3d) -> Vec3d {
        Vec3d {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
    fn dot(&self, other: &Vec3d) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    fn magsq(&self) -> f64 {
        self.x*self.x + self.y*self.y + self.z*self.z
    }

    fn abs(&self) -> f64 {
        self.magsq().sqrt()
    }

    fn normalize(&self) -> Vec3d {
        let m = self.abs();
        Vec3d {
            x: self.x / m,
            y: self.y / m,
            z: self.z / m,
        }
    }

    fn cross(&self, other: &Vec3d) -> Vec3d {
        Vec3d {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

fn main() {
    let v1 = Vec3d { x: 1.0, y: 1.0, z: 1.0 };
    let v2 = Vec3d { x: 2.0, y: 1.0, z: 0.0 };

    let v3 = v1.add(&v2);
    let v4 = v1.sub(&v2);
    let v5: Vec3d = v3; //v1 + v2;
    let v6: Vec3d = v4; //v1 - v2;
    let m = v1.magsq();
    let a = v1.abs();
    let v8 = v1.normalize();
    let v9 = v1.cross(&v2);
    println!("v1={:?}", v1);
    println!("v2={:?}", v2);
    println!("v3={:?}", v3);
    println!("v4={:?}", v4);
    println!("v5={:?}", v5);
    println!("v6={:?}", v6);
    println!("dot={:?}", v1.dot(&v2));
    println!("m={}", m);
    println!("a={}", a);
    println!("v8={:?}", v8);
    println!("v9={:?}", v9);
}
