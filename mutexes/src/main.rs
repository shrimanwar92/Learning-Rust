use std::io;
use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;

// number of threads we want to spawn
const N: i32 = 10;

fn main() {
    let mut input = String::new();

    println!("Please enter a number.");

    let res = match io::stdin().read_line(&mut input) {
    	Ok(_) => input.trim_end(),
    	Err(e) => panic!("{:?}", e)
    };

    let res = res.parse::<u32>().unwrap();
    println!("This is the number: {:?}",res);

    let data = Arc::new(Mutex::new(res));
    let (tx, rx) = channel();

    for i in 0..N {
    	let (data, tx) = (data.clone(), tx.clone());

    	thread::spawn(move || {
    		let mut data = data.lock().unwrap();
    		*data += 1;
    		tx.send(i).unwrap();
    	});
    }

    for _ in 0..N {
        println!("Got data from thread: {:?}", rx.recv().unwrap());
    }

    println!("{:?}", data);

}
