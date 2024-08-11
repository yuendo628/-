use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let path = "sysctl.conf";
    let file = File::open(path).unwrap();
    let reader: BufReader<File> = BufReader::new(file);
    let mut v: Vec<String> = Vec::new();

    for line in reader.lines() {
        let conftxt = line.unwrap();

        if conftxt.contains("kernel.endpoint") {
            v.push(conftxt);
        }
        else if conftxt.contains("kernel.debug") {
            v.push(conftxt);
        }
        else if conftxt.contains("kernel.log.file") {
            v.push(conftxt);
        }
    }

    println!("{}", v[0]);
    println!("{}", v[1]);
    println!("{}", v[2]);
    }