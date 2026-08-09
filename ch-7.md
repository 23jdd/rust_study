# 包、Crates 与模块

Rust 模块系统用于组织代码：包（Packages）、Crate、模块（Modules）、路径（Paths）、`use` 关键字。

## 包和 Crate
- **Crate**：一个模块树，是编译器一次处理的最小代码单元。分为二进制 crate（`src/main.rs`）和库 crate（`src/lib.rs`）。
- **包（Package）**：一个或多个 crate，提供一组功能。包含 `Cargo.toml`。

## 定义模块
```rs
// src/lib.rs
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}
```

## 路径与私有性
```rs
// 默认所有项（函数、方法、结构体、枚举、模块、常量）都是私有的
// 父模块不能使用子模块的私有项，子模块可以使用祖先模块的项
// 使用 pub 关键字公开

mod front_of_house {
    pub mod hosting {           // pub 使模块可被外部访问
        pub fn add_to_waitlist() {}  // pub 使函数公开
    }
}

pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();
    // 相对路径
    front_of_house::hosting::add_to_waitlist();
}
```

## use 关键字
```rs
mod front_of_house { pub mod hosting { pub fn add_to_waitlist() {} } }

use crate::front_of_house::hosting;
// use std::collections::HashMap;
// use std::{cmp::Ordering, io};   // 嵌套路径

hosting::add_to_waitlist();

// pub use 重导出
pub use crate::front_of_house::hosting;

// 外部包：在 Cargo.toml 添加依赖后 use
// use rand::Rng;
```

## 模块拆分到多个文件
```rs
// src/lib.rs
mod front_of_house;  // 加载 src/front_of_house.rs 或 src/front_of_house/mod.rs

// src/front_of_house.rs
pub mod hosting;     // 加载 src/front_of_house/hosting.rs
```
