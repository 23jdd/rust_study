use std::os;
use std::ptr;
use std::mem;
use std::alloc;

fn main(){
    let mut num = 5;
    let r1 = &num as *const i32;   // 不可变裸指针
    let r2 = &mut num as *mut i32; // 可变裸指针
    unsafe {
        println!("r1 is: {}", *r1);
        *r2 += 1;
        print!("{}",*r1)
    }
    
}