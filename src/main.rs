use std::fs::File;
use std::io::BufRead;
use std::{io, env};
use std::path::Path;

const SIGN_BEGIN:&str = "<!-- BEGIN LGBT-CN SIGNATURE -->";
const SIGN_END:&str = "<!-- END LGBT-CN SIGNATURE -->";
const COUNT_BEGIN:&str = "<!-- BEGIN LGBT-CN COUNT -->";
const COUNT_END:&str = "<!-- END LGBT-CN COUNT -->";

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        eprintln!("[Error] Wrong number of arguments. Request 1, provided {}", args.len() - 1);
    }
    let count: i16;
    let target = args[1].clone();
    if target.starts_with("http") {
        unimplemented!("TODO: URL DOWNLOAD");
        count = 0;
    } else {
        let content = file_read_lines(target).unwrap();
        count = get_sign_number(content);
    }

    println!("Count: {}", count);
}

fn get_sign_number(data: io::Lines<io::BufReader<File>>) -> i16 {
    let mut counter: i16 = -1;
    for line in data {
        match &line.unwrap().trim()[..] {
            SIGN_BEGIN => counter = 0,
            SIGN_END => break,
            "" => continue,
            _ => {
                if counter < 0 {
                    continue;
                } else {
                    counter += 1;
                }
            }
        };
    };
    counter
}

fn file_read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}