use std::io;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> io::Result<()> {   //
    loop {                               // infinite loop until killed 
        let mut input = String::new();  // mutable input string

    let result = io::stdin().read_line(&mut input); //reading number of bytes - unwrap gives value back  good for error
    let file = File::open("result.txt")?;
    let reader = BufReader::new(io::stdin());
    
    match result{
        Ok(0) => break,
        Ok(x) => println!("buffer: {}", x),
        Err(e) => println!("Error: {}", e),
    };


    println!("You typed: {}", input.trim());

    }

    
    Ok(())

}
