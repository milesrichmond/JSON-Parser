mod json;

use json::create_json;

fn file_to_str<'a>(filepath: &'a str) -> Result<String, String> {
    let file = std::fs::read_to_string(filepath);
    
    match file {
        Ok(contents) => {
            return Ok(contents);
        },
        Err(error_msg) => {
            return Err(error_msg.to_string());
        }
    }
}

fn main() {
    let mut args = std::env::args();
    
    if args.len() > 2 || args.len() < 1 {
        println!("Invalid argument count!, found: {} args when 2 expected", args.len());
        return;
    }
    
    let filename: String = args.nth(1).unwrap();
    let json = file_to_str(filename.as_str());
    
    if let Err(error) = json {
        println!("{filename}: {}, your current working directory is {}", error, std::env::current_dir().unwrap().as_path().display());
        return;
    }
    
    let head = create_json(json.as_ref().unwrap());
    
    if let Ok(head_object) = head {
        println!("{}", head_object)
    } else if let Err(error) = head {
        println!("{}", error);
        println!("Given: {}", json.unwrap());
    }
}
