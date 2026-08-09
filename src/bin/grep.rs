use std::env;
use std::fs;
use std::io;

fn read_all(path: &str) -> io::Result<String> {
    let content = fs::read_to_string(path)?;
    io::Result::Ok(content)
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("args number is not enough")
    }
    let file_name = &args[1];
    let query = &args[2];
    let content = read_all(&file_name);
    match content {
        Ok(v) => {
            let ok = v.contains(query);
            if ok {
                println!("yes")
            }
        }
        Err(_) => {
            eprintln!("READ {} FAIL", file_name)
        }
    }
}
