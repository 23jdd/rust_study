use std::thread::sleep;

fn fbnc(n: i32) -> i32 {
    if n == 0 || n == 1 {
        1
    } else {
        fbnc(n - 1) + fbnc(n - 2)
    }
}

fn main() {
    // print 1-10 斐波那契数。
    for i in 1..=10 {
        sleep(std::time::Duration::new(1,0)); 
        println!("{}:{}", i, fbnc(i))
    }
}
