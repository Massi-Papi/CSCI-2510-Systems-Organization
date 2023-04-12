use std::str;

fn main() {
//"Hello"encoded in UTF-8
    let bytes : [u8; 7] = [ 0x48, 0x65, 0x6C, 0x6C, 0x6F];
    let copied_bytes = bytes.to_vec();

    let view_into_bytes = &bytes[0..3]; //normally called a slice

    let result = str::from_utf8( &copied_bytes );
    if result.is_ok(){
        println!("Uncle {}", string_result);
    }
    // if result.is_ok(){
    //     let string_result = result.unwrap().to_string();
    //     println!("Found UTF characters {}", string_result.bytes().count());
    // } else if result.is_err() {
    //     println!("Provided bytes were not valid UTF-8");
    // }

}
