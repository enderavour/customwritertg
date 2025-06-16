use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::process::exit;
use std::collections::HashMap;


pub fn parse_settings() -> Result<HashMap<String, String>, Box<dyn Error>>
{
    let fd = File::open("settings").unwrap_or_else(|e | {
        eprintln!("Error parsing settings file: {e}");
        exit(-1);
    });

    let bufr = BufReader::new(fd);

    let mut hashmap = HashMap::new();

    for line in bufr.lines()
    {
        let unwrapped = line?;
        let mut substrs = unwrapped.split_whitespace();

        let (property, value) = (substrs.next(), substrs.next());

        if let Some(prop) = property
        {
            match prop
            {
                "API_ID" | "API_HASH" | "PHONENUMBER" => 
                hashmap.insert(prop.to_owned(), value.unwrap_or("").to_owned()),
                _ => Some(String::new())
            };
        }
    }
    Ok(hashmap)
}