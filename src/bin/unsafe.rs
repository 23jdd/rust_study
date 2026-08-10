use std::alloc;
use std::mem;
use std::os;
use std::ptr;
use std::slice;
fn main() {
    let mut num = 5;
    let r1 = &num as *const i32; // 不可变裸指针
    let r2 = &mut num as *mut i32; // 可变裸指针
    unsafe {
        println!("r1 is: {}", *r1);
        *r2 += 1;
        print!("{}", *r1)
    }
}

fn split_at_mut<'a>(values: &'a mut [i32], mid: usize) -> (& 'a mut [i32], & 'a mut  [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();
    assert!(mid <= len);
    // 安全抽象包装 unsafe
    unsafe {
        (   
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
