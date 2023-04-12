use std::io;

fn main() -> io::Result<()> {   //
    loop {                               // infinite loop until killed 
        let mut input = String::new();  // mutable input string

    let bytes = io::stdin().read_line(&mut input).unwrap(); //reading number of bytes - unwrap gives value back  good for error

    println!("You typed: {}", input.trim());

    if bytes == 0{
        break;
    }
    }
    
    Ok(())

}
