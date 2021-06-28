fn main() {
    let length = 5;
    let width = 10;
    println!("area({}, {}) => {}", length, width, area(length, width));

    let rect1 = (10, 10);
    println!("area_tuple(rect1) => {}", area_tuple(rect1));

    let rect2 = Rectangle {
        length: 20,
        width: 20,
    };
    println!("rect2.area() => {}", rect2.area());

    let rect3 = Rectangle {
        length: 5,
        width: 5,
    };
    println!("rect2.can_hold(rect3) => {}", rect2.can_hold(&rect3));

    let rect4 = Rectangle {
        length: 25,
        width: 5,
    };
    println!("rect3.can_hold(rect4) => {}", rect3.can_hold(&rect4));

    let square = Rectangle::square(2);
    println!("{:?}", square);
}

fn area(length: u32, width: u32) -> u32 {
    length * width
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn square(side: u32) -> Rectangle {
        Rectangle {
            length: side,
            width: side,
        }
    }

    fn area(&self) -> u32 {
        self.length * self.width
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.length >= rect.length && self.width >= rect.width
    }
}
