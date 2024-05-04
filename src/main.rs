struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 20,
        height: 60
    };

    println!("The area of rectangles is {} square pixels", rect1.area());
}
