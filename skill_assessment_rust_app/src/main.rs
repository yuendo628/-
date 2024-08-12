use std::fs::File;
use std::io::{BufRead, BufReader};

fn is_bool(value: String) -> bool {
    let mut result = false;

    if value.contains("TRUE") {
        result = true;
    }
    else if value.contains("FALSE")
    {
        result = true;
    }

    result
}

fn main() {
    let path = "sysctl.conf";
    let file = File::open(path).unwrap();
    let reader: BufReader<File> = BufReader::new(file);

    for line in reader.lines() {
        let conftxt = line.unwrap();
        if conftxt.contains("kernel.endpoint") {
                if is_bool(conftxt.clone()) == false  {
                    println!("{}", conftxt.clone());
            }
        }
        else if conftxt.contains("kernel.debug") {
            if is_bool(conftxt.clone()) {
                println!("{}", conftxt.clone());
            }
        }
        else if conftxt.contains("kernel.log.file") {
            if is_bool(conftxt.clone()) == false {
                println!("{}", conftxt.clone());
            }
        }
    }
}