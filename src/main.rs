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
    
    if args.by_ref().count() > 2 {
        println!("Too many arguments provided");
        return;
    }
    
    let json = file_to_str("test1.json");
    
    if let Err(error) = json {
        println!("{}, your current working directory is {}", error, std::env::current_dir().unwrap().as_path().display());
        return;
    }
    
    //let head = JsonObj::try_from(json.unwrap().as_str());
    let head = create_json(json.as_ref().unwrap());
    
    if let Ok(head_object) = head {
        println!("{}", head_object)
    } else if let Err(error) = head {
        println!("{}", error);
        println!("Given: {}", json.unwrap());
    }
}
