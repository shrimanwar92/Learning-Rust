#![allow(dead_code)]

enum WebEvent {
	PageLoad,
	PageUnload,
	KeyPress(char),
	Paste(String),
	Click {x: i64, y: i64}
}

fn inspect(event: WebEvent) {
	match event {
		WebEvent::PageLoad => println!("Page loaded"),
		WebEvent::PageUnload => println!("page unloaded"),
		WebEvent::KeyPress(c) => println!("pressed '{:?}'.", c),
		WebEvent::Paste(p) => println!("pasted \"{:?}\". ", p),
		WebEvent::Click {x, y} => {
			println!("clicked at x={:?}, y={:?}.", x, y);
		},
	}
}

fn main() {
    let pl = WebEvent::PageLoad;
    let pul = WebEvent::PageUnload;
    let kp = WebEvent::KeyPress('x');
    let p = WebEvent::Paste("this is a text".to_owned());
    let c = WebEvent::Click {x: 20, y: 20};

    inspect(pl);
    inspect(pul);
    inspect(kp);
    inspect(p);
    inspect(c);
}
