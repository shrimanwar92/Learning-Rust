#[allow(dead_code)]
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug)]
struct FileData {
	name: &'static str,
	contents: &'static str 
}

impl FileData {
	fn create(&self) {
		let pathName = [self.name, "txt"].join(".");
		let path = Path::new(&pathName);
		let display = path.display();

		// Open a file in write-only mode, returns `io::Result<File>`
		let mut file = match File::create(&path) {
			Err(why) => panic!("couldn't create {}: {}", display, why.description()),
			Ok(file) => file,
		};

		let result = match file.write_all(self.contents.as_bytes()) {
			Err(why) => {
            	panic!("couldn't write to {}: {}", display,
                                               why.description())
        	},
        	Ok(_) => println!("successfully wrote to {}", display),
		};
	}
}

fn main() {
    let a = FileData {
    	name: "nilay",
    	contents: "test contents",
    };

    a.create()
}
