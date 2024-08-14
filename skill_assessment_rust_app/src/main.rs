use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use std::io::ErrorKind;

fn converttostring(mut value: String) -> String {

    if value == "1" {
        value = "true".to_string();
    }
    else if value == "0" {
        value = "false".to_string();            
    }

    value
}

fn main() {
    let path = "sysctl.conf";
    let file = match File::open(path) {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => panic!("ファイルが見つかりませんでした。"),
            _ => panic!("ファイルのオープンに失敗しました： {:?}", error),
        }
    };
    let reader: BufReader<File> = BufReader::new(file);
    let mut map: HashMap<String, String> = HashMap::new();

    for line in reader.lines() {
        let conftxt = line.unwrap();

        if conftxt.contains("#"){
            continue;
        }
        else if conftxt.is_empty() {
            continue;            
        }

        let v: Vec<&str> = conftxt.split('=').collect::<Vec<&str>>();
        let mut value: String = v[1].chars().filter(|c| !c.is_whitespace()).collect();
        
        let keys: Vec<&str> = v[0].split('.').collect::<Vec<&str>>();
        let key: &str = keys.last().unwrap();

        value = converttostring(value);
        map.insert(String::from(key.to_string()), String::from(value));
        println!("{}:{}", key, map.get(key).unwrap());    
    }
}