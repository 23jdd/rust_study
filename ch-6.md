# 枚举和模式匹配

枚举（enum）用于定义某个类型所有可能的变体（variants）。

## 枚举的定义
```rs
enum IpAddrKind {
    V4,
    V6,
}

// 枚举变体可以关联数据
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}
let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));

// 带多种数据类型的枚举
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

## Option<T> 枚举
```rs
// 标准库定义（已自动导入，无需 use）
// enum Option<T> { None, Some(T) }

let some_number = Some(5);
let some_char = Some('e');
let absent_number: Option<i32> = None;
// Option<T> 和 T 是不同的类型，不能直接运算
```

## match 控制流
```rs
enum Coin { Penny, Nickel, Dime, Quarter }

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

// 匹配 Option<T>
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

// match 必须穷尽所有可能，可以用 _ 作为通配符
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => reroll(),  // 其他所有情况
}
```

## if let 和 let else
```rs
let config_max = Some(3u8);
// if let 简洁控制流
if let Some(max) = config_max {
    println!("最大值是 {}", max);
}

// let else —— 不匹配则提前返回/退出
let Some(max) = config_max else {
    return;
};
```
