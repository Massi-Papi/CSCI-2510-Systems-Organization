//use std::time::Duration;
use std::time::Instant;
const TRIALS: u32 = 100;

fn add( a:i32, b:i32){
    for _ in 0..TRIALS{
        let _ = a + b;
}
}

fn main() {
    let start = Instant::now();
    println!("Hello World");
	let end = Instant::now();
        let runtime = end.duration_since(start);
	println!("Running time: {:?}", runtime);
}
