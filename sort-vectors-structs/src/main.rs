
fn sort_vector() {
	let mut v = vec![1, 5, 10, 2, 15];
	v.sort();
	assert_eq!(v, vec![1, 2, 5, 10, 15]);
	println!("{:?}", v);
}

fn sort_structs() {
	#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
	struct Person {
		name: String,
		age: u32
	}

	impl Person {
		fn new(name: String, age: u32) -> Self {
			Person {
				name,
				age
			}
		}
	}

	let mut people = vec![
		Person::new("nilay".to_string(), 30),
		Person::new("mohan".to_string(), 19),
		Person::new("test".to_string(), 54)
	];

	people.sort();

	// default sort by name
	println!("{:?}", people);

	// sort by age
	people.sort_by(|a, b| b.age.cmp(&a.age));
	println!("{:?}", people);
}

fn main() {
    sort_vector();
    sort_structs();
}
