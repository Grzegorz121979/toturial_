use std::io::{self, Write};

/* struct Rectangle {
    width: u32,
    height: u32
} */

fn main() {
    /* let rect1 = Rectangle {
        width: 30,
        height: 50
    }; */
    
    let mut w = String::new();
    let mut h = String::new();

    print!("Width: ");
    io::stdout()
        .flush()
        .unwrap();
    io::stdin()
        .read_line(&mut w)
        .expect("Failed read line");

    let w: u32 = w.trim()
                    .parse()
                    .unwrap();

    print!("Height: ");
    io::stdout()
        .flush()
        .unwrap();
    io::stdin()
        .read_line(&mut h)
        .expect("Failed read line");

    let h: u32 = h.trim()
                    .parse()
                    .unwrap();
    
    println!("The area of rectangle is {} square pixels", area(w, h));
}

fn area(a: u32, b: u32) -> u32 {
    a * b
}

/* fn area(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
} */
