use config::Config;
use std::fs::{File, OpenOptions};
use std::io::ErrorKind::NotFound;
use std::io::ErrorKind::WouldBlock;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::time::Duration;
use std::{thread, time};


fn main() {

    let config_path = "../config.toml";

    let settings = match Config::builder()
        .add_source(config::File::with_name(config_path))
        .build() {
            Ok(x) => x,
            Err(x) => panic!("Could not open configuration file: {x}"),
        };


    let log_ip = settings.get_string("log_ip").unwrap();
    let log_port = settings.get_string("log_port").unwrap();
    let log_file_str = settings.get_string("log_file").unwrap();
    // while ... {
    //     ...
    //     let mut file = match OpenOptions::new()
    //         .create(true)
    //         .append(true)
    //         .open(&log_file_str)
    //         ...
    // }
    

    let connect_server = format!("{}:{}", log_ip, log_port);
    let listener = TcpListener::bind(connect_server).unwrap();
    let mut clients = Vec::new();

    //GRADING NOTE: Why are you binding 8080? You arlready have a listener
    // up above. Moreover, you overwrite 'listener' so your server never
    // checks that other stream, -5 points
    //let listener = TcpListener::bind("0.0.0.0:8080").expect("Failed to bind listener");  
    listener
        .set_nonblocking(true)
        .expect("Failed to set non-blocking");

    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                stream
                    .set_nonblocking(true)
                    .expect("Failed to set non-blocking");
                clients.push(stream);
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                
                continue;
            }
            Err(e) => panic!("Error accepting incoming connection: {}", e),
        }
    }

    //GRADING NOTE: for listener.incoming() will never terminate until the
    // listening socket is destroyed, that style of programming is fundamentally
    // incompatible with non-blocking programming, as discussed in class
    // -10 points
    println!("You never get here, code after here never runs");
    
    let mut i = 0;
    while i < clients.len() {
        let mut buffer = [0; 1024];
        match clients[i].read(&mut buffer) {
            Ok(num_bytes) => num_bytes,
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                
                continue;
            }
            //GRADING NOTE: The following error represents an unrecoverable
            // error for this client, and should remove that client from the
            // clients list. -5 points
            Err(e) => panic!("Error reading from client: {}", e),
        };

        //GRADING NOTE: This opens a file for every message on every connection,
        // which isn't efficient and won't give correct results. -5 points
        let mut file = match OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_file_str.clone())
        {
            Ok(file) => file,
            Err(ref e) if e.kind() == std::io::ErrorKind::NotFound => {
                
                //GRADING NOTE: Cloning here creates a whole copy of the
                // string, and you're just trying to avoid ownership problems-
                // instead, use a reference
                // GRADING NOTE: Also, I'm not sure what this is trying
                // to accomplish versus the above call. If the above call
                // fails, wouldn't this one too?
                match File::create(log_file_str.clone()) {
                    Ok(mut new_file) => {
                        new_file.write_all(&buffer[..]).unwrap();
                        new_file
                    }
                    Err(e) => panic!("Error creating log file: {}", e),
                }
            }
            Err(e) => panic!("Error opening log file: {}", e),
        };

        match file.write_all(&buffer[..]) {
            Ok(_) => (),
            Err(e) => panic!("Error writing to log file: {}", e),
        };

        i += 1;
        thread::sleep(Duration::from_millis(250));
    }
}

