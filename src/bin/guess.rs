use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("guess game");
    // 生成 1-100 随机数
    let random_number = rand::thread_rng().gen_range(1..=100);
    loop {
        let mut guess = String::new();
        io::stdin().read_line(& mut guess).expect("IO ERROR");
        let guess = match guess.trim().parse::<i32>() {
            Ok(v) => v,
            Err(_) => {
                eprintln!("Please Input a number");
                continue;
            }
        };
        match guess.cmp(&random_number) {
            Ordering::Equal => {
                println!("you are right");
                break;
            }
            Ordering::Greater => {
                println!("is greater");
            }
            Ordering::Less => {
                println!("is less") 
            }
        }
    }
}
