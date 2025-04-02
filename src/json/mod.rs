mod json_object;
mod json_parser;

use json_object::JsonObject;
use json_parser::{is_array, is_string, parse_array, parse_string, parse_boolean, parse_number};

/// Creates a JsonObject from a supplied json string.
/// 
/// Err if the supplied string does not appear to be a json array
pub fn create_json(untrimmed_json_str: &str) -> Result<JsonObject, String> {
    if is_array(untrimmed_json_str) {
        return parse_array(untrimmed_json_str.trim());
    }
    
    Err(String::from("Uppermost JSON should be an array"))
}

/// Not publically available, since the uppermost layer of json MUST be an array.
/// This function should only be used when constructing that array.
/// 
/// When creating a JsonObject to match a json file, the public version of this function
/// (create_json) must be called instead.
fn create_json_recursive(untrimmed_json_str: &str) -> Result<JsonObject, String> {
    let json_str: &str = untrimmed_json_str.trim();

    if is_array(json_str) {
        return parse_array(json_str)
    } else if is_string(json_str) {
        return parse_string(json_str)
    } else if let Ok(num) = parse_number(json_str) {
        return Ok(num)
    } else if let Ok(bool) = parse_boolean(json_str) {
        return Ok(bool)
    } else if json_str == "null" {
        return Ok(JsonObject::Null())
    } else {
        let mut err_msg = String::from("No valid JSON detected!");
        err_msg.push_str(&format!("\nGiven: {}", json_str));
        return Err(err_msg);
    }
}