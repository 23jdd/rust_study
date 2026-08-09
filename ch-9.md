# 错误处理

Rust 将错误分为两类：可恢复错误（`Result<T, E>`）和不可恢复错误（`panic!`），没有异常机制。

## panic! 不可恢复错误
```rs
// 直接调用
panic!("crash and burn");

// 默认展开栈（unwinding），可在 Cargo.toml 设置立即终止
// [profile.release]
// panic = 'abort'

// 设置环境变量获取回溯：RUST_BACKTRACE=1
```

## Result<T, E> 可恢复错误
```rs
use std::fs::File;
use std::io::ErrorKind;

// 基本处理
let greeting_file_result = File::open("hello.txt");
let greeting_file = match greeting_file_result {
    Ok(file) => file,
    Err(error) => match error.kind() {
        ErrorKind::NotFound => match File::create("hello.txt") {
            Ok(fc) => fc,
            Err(e) => panic!("创建文件失败: {:?}", e),
        },
        other_error => panic!("打开文件失败: {:?}", other_error),
    },
};

// 简写：unwrap（成功返回值，失败 panic）
let greeting_file = File::open("hello.txt").unwrap();

// expect：类似 unwrap，但可自定义错误信息
let greeting_file = File::open("hello.txt")
    .expect("hello.txt 应该包含在项目中");
```

## 传播错误（? 运算符）
```rs
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;  // 出错则提前返回 Err
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// 链式调用
fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

// 更简洁：标准库已有
// std::fs::read_to_string("hello.txt")

// ? 只能用于返回 Result/Option 的函数
// main 函数也可以返回 Result：
// fn main() -> Result<(), Box<dyn std::error::Error>> { ... }
```

## 何时 panic
- 示例、原型、测试中适合 unwrap/expect
- 比编译器知道更多信息时（如已知不会 Err）
- 调用外部代码传入无效值时
- 进入坏状态（bad state）且无法恢复时
