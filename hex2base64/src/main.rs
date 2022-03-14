use hex;
use base64;

pub fn convert_hex_to_base64(hex: &str) -> String {
    let bytes = hex::decode(hex).unwrap();
    return base64::encode(bytes);
}


pub fn main(){
    println!("[-] Convert HEX to BASE64 - cryptopals.com [-]");
    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    println!("[*] Input value: {}", input);
    let base64 = convert_hex_to_base64(input);
    println!("[*] Output value: {}", base64);
}
