use std::{collections::HashMap, fmt};

pub enum JsonObject {
    Array(HashMap<String, JsonObject>),
    String(String),
    Boolean(bool),
    Number(JsonNumber),
    Null()
}

impl JsonObject {
    pub fn string(self) -> Option<String> {
        if let JsonObject::String(value) = self {
            Some(value)
        } else {
            None
        }
    }
}

impl fmt::Display for JsonObject {    
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Array(map) => {
                let _ = write!(f, "{{\n");
                for key in map.keys() {
                    let result = write!(f, "{} = {}\n", key, map[key]);
                    if let Err(error) = result {
                        return Err(error)
                    }
                }
                let _ = write!(f, "}}");
                
                Ok(())
            },
            Self::String(string) => {
                write!(f, "{}", string)
            },
            Self::Boolean(boolean) => {
                if *boolean {
                    write!(f, "true")
                } else {
                    write!(f, "false")
                }
            },
            Self::Number(number) => {
                write!(f, "{}", number)
            },
            Self::Null() => {
                write!(f, "null")
            }
        }
    }
}

pub enum JsonNumber {
    Float(f32),
    Integer(i32)
}

impl fmt::Display for JsonNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Integer(value) => write!(f, "{}", value),
            Self::Float(value) => write!(f, "{}", value)
        }
    }
}