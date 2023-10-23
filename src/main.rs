use serde_json;
use std::env;

fn decode_dencode_value(encoded_value: &str) -> serde_json::Value {
    if encoded_value.chars().next().unwrap().is_digit(10) {
        let colon_index = encoded_value.find(":").unwrap();
        let number_string = &encoded_value[..colon_index];
        let number = number_string.parse::<i64>().unwrap();
        let string = &encoded_value[colon_index + 1..colon_index + 1 + number as usize];
        return serde_json::Value::String(string.to_string());
    } else {
        panic!("Unhandled encoded value: {}", encoded_value)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        println!("Logs");

        let encoded_value = &args[2];
        let decoded_value = decode_dencode_value(encoded_value);
        println!("{}", decoded_value.to_string())
    } else {
        println!("Unknown command: {}", args[1])
    }
}
