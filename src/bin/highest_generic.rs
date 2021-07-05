fn main() {
    let v = vec![1, 2, 3, 4, 5];
    println!("highest: {}", highest(&v));

    let v = vec![5, 4, 3, 2, 1];
    println!("highest: {}", highest(&v));

    let v = vec![5];
    println!("highest: {}", highest(&v));

    let v = vec![4, 5, 2, 1, 3];
    println!("highest: {}", highest(&v));

    let v = vec!['a', 'b', 'c', 'd', 'e'];
    println!("highest: {}", highest(&v));

    let p = Point { x: 1, y: 2 };
    println!("p.x => {}", p.x());

    let p = Point { x: 1.0, y: 2.0 };
    println!("p.distance_from_origin() => {}", p.distance_from_origin());
}

fn highest<T: PartialOrd>(v: &[T]) -> &T {
    let mut highest = &v[0];
    for i in v {
        if i > highest {
            highest = &i;
        }
    }
    &highest
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
