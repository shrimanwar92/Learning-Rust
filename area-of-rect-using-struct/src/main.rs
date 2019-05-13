use std::env;

#[derive(Debug)]
struct Rectangle {
	length: u32,
	breadth: u32
}

impl Rectangle {
	fn area(&self) -> u32 {
		self.length * self.breadth
	}

	fn can_hold(&self, other_rect: &Rectangle) -> bool{
		self.area() > other_rect.area()
	}
}

fn main() {
	let args: Vec<String> = env::args().collect();
    let rect1 = Rectangle { length: args[1].parse::<u32>().unwrap(), breadth: args[2].parse::<u32>().unwrap()};
    let rect2 = Rectangle { length: args[3].parse::<u32>().unwrap(), breadth: args[4].parse::<u32>().unwrap()};
    let rect3 = Rectangle { length: args[5].parse::<u32>().unwrap(), breadth: args[6].parse::<u32>().unwrap()};
    

    println!("Area of rectangle is: {:?}", rect1.area());

    /*let rect1 = Rectangle { length: 50, breadth: 30 };
	let rect2 = Rectangle { length: 40, breadth: 10 };
	let rect3 = Rectangle { length: 45, breadth: 60 };*/

	println!("Area of rectangle1 is: {:?}", rect1.area());
	println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
	println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

}
