use std::collections::HashMap;


#[derive(Debug)]
struct NumberToWords {
	number: i32
}

impl NumberToWords {
	fn get_dictionary() -> HashMap<&'static str, &'static str> {
		let mut dictionary = HashMap::new();
		dictionary.insert("1", "one");
		dictionary.insert("2", "two");
		dictionary.insert("3", "three");
		dictionary.insert("4", "four");
		dictionary.insert("5", "five");
		dictionary.insert("6", "six");
		dictionary.insert("7", "seven");
		dictionary.insert("8", "eight");
		dictionary.insert("9", "nine");
		dictionary.insert("10", "ten");
		dictionary.insert("11", "eleven");
		dictionary.insert("12", "twelve");
		dictionary.insert("13", "thirteen");
		dictionary.insert("14", "fourteen");
		dictionary.insert("15", "fifteen");
		dictionary.insert("16", "sixteen");
		dictionary.insert("17", "seventeen");
		dictionary.insert("18", "eighteen");
		dictionary.insert("19", "nineteen");
		dictionary.insert("20", "twenty");
		dictionary.insert("30", "thirty");
		dictionary.insert("40", "fourty");
		dictionary.insert("50", "fifty");
		dictionary.insert("60", "sixty");
		dictionary.insert("70", "seventy");
		dictionary.insert("80", "eighty");
		dictionary.insert("90", "ninety");
		dictionary.insert("100", "one hundred");

		dictionary
	}

	fn new(num: i32) -> NumberToWords {
		NumberToWords {
			number: num
		}
	}

	fn make_word(&self) -> Option<&str>{
		let s: String = self.number.to_string();
		let dict = NumberToWords::get_dictionary();

		// mystring.chars().last().unwrap();

		match &s.len() {
			1 => match dict.get(s.as_str()) {
        			Some(val) => return Some(val),
        			None => return None,
    		},
    		/*2 => match dict.get(s.as_str()) {
    			Some(val) if dict.contains_key(val) => return Some(val),
    			Some(_) => return Some(),
    			_ => panic!("Please input proper input"),
    		}*/
    		2 => match NumberToWords::two_digits(s, &dict) {
    			Some(val) => return Some(val),
    			None => return None,
    		}
    		_ => return None
		}
	}

	fn two_digits(t: String, dict: &HashMap<&str, &str>) -> Option<String> {
		let s = t.to_string();

		println!("{:?}", s);

		let first: Vec<char> = s.chars().take(1).collect();
		let mut first: String = first.into_iter().collect();
		first.push_str(&"0".to_string());

		let last: Vec<char> = s.chars().rev().take(1).collect();
		let last: String = last.into_iter().collect();

		let res1 = dict.get(first.as_str());
		let res1 = match res1 {
			Some(s) => s,
			None => "not found",
		};
		let res2 = dict.get(last.as_str());
		let res2 = match res2 {
			Some(s) => s,
			None => "not found",
		};

		println!("{:?}", [res1, res2].join(" "));
		

		Some([res1, res2].join(" ").to_owned())
	}
}

fn main() {
	


	// println!("{:?}", dictionary.contains_key("109"));


    let w = NumberToWords::new(23);
    match w.make_word() {
    	Some(s) => println!("{:?}", s),
    	None => println!("Not found"),
    }
}
