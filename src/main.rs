trait Shape {
    fn area(&self) -> f64;
}

struct Circle {
    r: f64,
}

impl Circle {
    fn new(r: f64) -> impl Shape {
        return Circle{
            r: r,
        }
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        return self.r * self.r * 3.14;
    }
}

fn sum(shapes: &[impl Shape]) -> f64 {
    let mut result = 0.0;

    for s in shapes {
        result += s.area()
    }
    return result;
}


fn main() {
    let shapes = vec![Circle::new(10.0)];
    print!("{}", sum(&shapes));
}
