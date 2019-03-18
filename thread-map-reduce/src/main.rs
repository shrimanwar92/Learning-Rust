use std::thread;

const CHUNKS: usize = 5;

fn main() {
    // This is our data to process.
    // We will calculate the sum of all digits via a threaded  map-reduce algorithm.
    // Each whitespace separated chunk will be handled in a different thread.
    //
    // TODO: see what happens to the output if you insert spaces!
    let data = "8696789773741 6471853297327050364959
118613225755647 239632975426249 62850
7085623470186085 1907960690014 725 639
38397966707106094 17278323 8747 6692 19
523 807 952578 882365 25459 3033 30302 837
5849 5327 13574 40410 48897 88573429 7812
699 2021 6438980 8735 4880 8413 72095 6532
1627 842463 745258 98 6034 537 4 8 2857466 8";

	let mut children = vec![];

	let chunked_data = data.split_whitespace().collect::<Vec<_>>();

	for (i, chunk) in chunked_data.chunks(CHUNKS).enumerate() {
		let chunk = chunk.join("");

		children.push(thread::spawn(move || -> u32 {
			let result = chunk.chars()
							.map(|c| c.to_digit(10).expect("should be a digit"))
							.sum();

			println!("processed segment {}, result={}", i, result);

			result

		}));
	}

	let mut sums = vec![];

	for child in children {
		let sum = child.join().unwrap();
		sums.push(sum);
	}

	let final_result = sums.iter().sum::<u32>();
	println!("{:?}", final_result);
}
