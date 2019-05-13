use std::env;

#[derive(Debug)]
struct Rectangle {
	length: u32,
	breadth: u32
}

fn main() {
	let args: Vec<String> = env::args().collect();
    let rect = Rectangle { length: args[1].parse::<u32>().unwrap(), breadth: args[2].parse::<u32>().unwrap()};
    let result = area(&rect);
    println!("Area of rectangle is: {:?}", result);
}

fn area(rectangle: &Rectangle) -> u32 {
	rectangle.length * rectangle.breadth
}
