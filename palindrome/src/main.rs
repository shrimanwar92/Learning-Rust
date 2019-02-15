use std::env;

fn main() {

	if let Some(arg1) = env::args().nth(1) {
		let s = arg1.to_string();
		let rev = s.chars().rev().collect::<String>();

		println!("original string: {:?}", s);
    	println!("reversed string: {:?}", rev);
    	if s == rev {
    		println!("string is palindrome");
    	} else {
    		println!("string is not palindrome");
    	}
	} else {
		println!("Please provide a string");
	}
}
