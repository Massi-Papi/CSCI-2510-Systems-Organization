use std::str;

fn main() {
//"SLU" followed by a Grinning face emoji encoded in UTF-8
    let bytes : [u8; 7] = [ 0x53, 0x4C, 0x55, 0xF0, 0x9F, 0x98, 0x80 ];
    let copied_bytes = bytes.to_vec();

    let view_into_bytes = &bytes[0..3]; //normally called a slice

    let result = str::from_utf8( &copied_bytes );
    if result.is_ok(){
        let string_result = result.unwrap().to_string();
        println!("Found UTF characters {}", string_result.bytes().count());
    } else if result.is_err() {
        println!("Provided bytes were not valid UTF-8");
    }

}
