use std::env;
use std::fs;
use std::io;
use std::io::BufRead;

fn main()->io::Result<()>{
     let args = env::args().collect::<Vec<String>>();
     if args.len()==1{
          eprintln!("Args Miss")   
     }
     let file_name=&args[1];
     let reader = fs::File::open(file_name).unwrap();
     let bufio = io::BufReader::new(reader);
     for (line,ele) in bufio.lines().enumerate() {
         let ele=ele?;
         println!("{}: {}",line,ele)
     }
     io::Result::Ok(())
}

