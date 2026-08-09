# 认识所有权
1. Rust 中的每一个值都有一个 所有者（owner）。
2. 值在任一时刻有且只有一个所有者。
3. 当所有者离开作用域，这个值将被丢弃。
```rs
let a=String::from("Owner");
let b=a;
println!("{}",a) // error  move
```
## explain
为什么,如果move后a还可用就会double free,这有点像c++的 `std::move`
