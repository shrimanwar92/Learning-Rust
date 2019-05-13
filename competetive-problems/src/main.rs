fn main() {
    let a = vec![-1, 7, 8, -5, 4];

    for (i, value) in a.iter().enumerate() {

    	if value < &0 {
    		continue;
    	} else {
    		println!("{:?}, {:?}", value, &a[(i+2) as usize]);
    	}
    }
}
