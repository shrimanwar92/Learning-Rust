use std::ops::Deref;

#[derive(Debug)]
struct MyBox<T>(T);

impl<T> MyBox<T> {
	fn new(x: T) -> MyBox<T> {
		MyBox(x)
	}
}

impl<T> Deref for MyBox<T> {
	type Target = T;

	fn deref(&self) -> &T {
		&self.0
	}
}

fn deref_trait_for_box_type() {
	let x= 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);

    // MyBox cannot be dereferrenced like primitive type
    // need to implement deref trait to deref a Box type
    assert_eq!(5, *y);
    println!("{:?}", *y);
}

fn for_string(name: &str) {
	println!("Hello, {:?}", name);
}

fn main() {
    deref_trait_for_box_type();
    
    let m = MyBox::new(String::from("Rust"));
    for_string(&m);
}
