use super::{create_json_recursive, json_object::JsonNumber, JsonObject};
use std::collections::HashMap;

/// Evaluates if the supplied array is a valid json array
pub fn is_array(json: &str) -> bool {
    if let Ok(removed) = remove_prefix_suffix(json.trim(), "{", "}") {
        let left_bracket_count = removed.matches("{").count();
        let right_bracket_count = removed.matches("}").count();
        return left_bracket_count - right_bracket_count == 0;
    }
    
    false
}

/// Evaluates if the supplied string is a valid json string
pub fn is_string(json: &str) -> bool {
    if let Ok(removed) = remove_prefix_suffix(json.trim(), "\"", "\"") {
        let escapes_removed = removed.replace("\\\"", "");
        return !escapes_removed.contains("\"");
    }
    
    false
}

/// 
pub fn parse_array(json: &str) -> Result<JsonObject, String> {
    let interior = remove_prefix_suffix(json, "{", "}").expect("parse_array was not given a valid json array");
    let mut hm: HashMap<String, JsonObject> = HashMap::new();
    
    // For each value, attempt to create a map entry
    for object in interior.trim().split(",") {
        if let Some((key_str, value_str)) = object.trim().split_once(":") {
            let key_json = parse_string(key_str);
            if let Err(key_error) = key_json {
                return Err(key_error)
            }
            
            let key = key_json.unwrap().string();
            if let None = key {
                panic!("Could not retrieve string value from JsonObject::String, this should not happen");
            }
            
            let object = create_json_recursive(value_str);
            if let Err(object_error) = object {
                return Err(object_error);
            }
            
            if let Some(_) = hm.insert(key.unwrap(), object.unwrap()) {
                return Err(String::from("Key already exists"));
            }
        }
    }
    
    Ok(JsonObject::Array(hm))
}

///
pub fn parse_string(json: &str) -> Result<JsonObject, String> {
    let trimmed_json = remove_prefix_suffix(json.trim(), "\"", "\"");
    if let Err(_) = trimmed_json {
        return Err(String::from("Missing enclosing quotes"))
    }
    
    Ok(JsonObject::String(String::from(trimmed_json.unwrap())))
}

///
pub fn parse_boolean(json: &str) -> Result<JsonObject, String> {
    match json {
        "true" => Ok(JsonObject::Boolean(true)),
        "false" => Ok(JsonObject::Boolean(false)),
        _ => Err(String::from("Boolean does not match pattern"))
    }
}

///
pub fn parse_number(json: &str) -> Result<JsonObject, String> {
    if let Ok(json_float) = json.parse::<f32>() {
        return Ok(JsonObject::Number(JsonNumber::Float(json_float)))
    } else if let Ok(json_int) = json.parse::<i32>() {
        return Ok(JsonObject::Number(JsonNumber::Integer(json_int)))
    }
    
    Err(String::from("No JSON number found!"))
}

/// Returns a str with the supplied prefix and suffix removed, if unsuccessful return Err
fn remove_prefix_suffix<'a>(string: &'a str, pre: &'a str, suf: &'a str) -> Result<&'a str, ()> {
    if let Some(no_suffix) = string.trim().strip_suffix(suf) {
        if let Some(removed) = no_suffix.strip_prefix(pre) {
            return Ok(removed)
        }
    }
    
    return Err(());
}