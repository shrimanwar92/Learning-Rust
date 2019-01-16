#[allow(dead_code)]
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::io;

#[derive(Debug)]
struct FileData {
	name: &'static str,
	contents: &'static str 
}

impl FileData {

	fn new(name: &'static str, contents: &'static str) -> FileData {
		FileData {
			name: name,
			contents: contents
		}
	}
	fn create(&self) -> Result<&'static str, &'static str> {
		let pathName = [self.name, "txt"].join(".");
		let path = Path::new(&pathName);
		let display = path.display();

		// Open a file in write-only mode, returns `io::Result<File>`
		let mut file = match File::create(&path) {
			Err(why) => panic!("couldn't create {}: {}", display, why.description()),
			Ok(file) => file,
		};

		match file.write_all(self.contents.as_bytes()) {
			Ok(file) => Ok("success"),
        	Err(err) => Err("fail"),
		}
	}
}

fn main() {
	static LOREM_IPSUM: &'static str =
		"Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod
		tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
		quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
		consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse
		cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non
		proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
		";
    
    let fd = FileData::new("123", LOREM_IPSUM);
    let res = fd.create();
    match res {
    	Ok(m) => println!("{:?}", m),
    	Err(e) => println!("{:?}", e),
    }
}
