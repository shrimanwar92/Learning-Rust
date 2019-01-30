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

	fn make_word(&self) -> Option<String>{
		let s = self.number.to_string();
		let dict = NumberToWords::get_dictionary();

		match s.len() {
			1 => match dict.get(s.as_str()) {
        			Some(val) => Some(val.to_string()),
        			None => None,
    		},
    		2 => match NumberToWords::two_digits(&s, &dict) {
    			Some(val) => Some(val),
    			None => None,
    		},
    		3 => match NumberToWords::three_digits(&s, &dict) {
    			Some(val) => Some(val),
    			None => None,
    		},
    		4 => match NumberToWords::four_digits(&s, &dict) {
    			Some(val) => Some(val),
    			None => None,
    		},
    		_ => return None
		}
	}

	fn two_digits(t: &str, dict: &HashMap<&str, &str>) -> Option<String> {
		let s = t.to_string();

		if dict.contains_key(&s.as_str()) {
			match dict.get(&s.as_str()) {
				Some(val) => return Some(val.to_string()),
				None => return None
			};
		}

		let first = &s[..1];
        let first = [first, "0"].join("");

		let res1 = dict.get(first.as_str());
		let res1 = match res1 {
			Some(s) => s.to_string(),
			None => "".to_string(),
		};

		let last = &s[1..];

		let res2 = dict.get(last);
		let res2 = match res2 {
			Some(s) => s.to_string(),
			None => "".to_string(),
		};	

		Some([res1, res2].join(" "))
	}

	fn three_digits(digits: &str, dict: &HashMap<&str, &str>) -> Option<String> {
		// 100
		// 200
		// 143
		// 120
		// 999
		let first = &digits[..1];

		let res1 = dict.get(first);
		let res1 = match res1 {
			Some(s) => s,
			None => "",
		};

		let last = &digits[1..];

		let res2 = NumberToWords::two_digits(&last, &dict);
		let res2 = match res2 {
			Some(s) => s.to_string(),
			None => "".to_string(),
		};

		if res1 != "" {
			Some([res1, "hundred", res2.as_str()].join(" "))
		} else {
			Some([res1, res2.as_str()].join(" "))
		}

	}

	fn four_digits(digits: &str, dict: &HashMap<&str, &str>) -> Option<String> {
		let first = &digits[..1];

		let res1 = dict.get(first);
		let res1 = match res1 {
			Some(s) => s,
			None => "",
		};

		let last = &digits[1..];

		let res2 = NumberToWords::three_digits(&last, &dict);
		let res2 = match res2 {
			Some(s) => s.to_string(),
			None => "".to_string(),
		};

		if res1 != "" {
			Some([res1, "thousand", res2.as_str()].join(" "))
		} else {
			Some([res1, res2.as_str()].join(" "))
		}
	}
}

fn main() {
    let w = NumberToWords::new(6111);
    match w.make_word() {
    	Some(s) => println!("{:?}", s),
    	None => println!("Not found"),
    }
}
