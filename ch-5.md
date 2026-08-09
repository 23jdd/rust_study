# 使用结构体组织相关联的数据
结构体（struct）是一种自定义数据类型，用于包装和命名多个相关的值。

## 结构体的定义和实例化
```rs
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};

// 结构体更新语法
let user2 = User {
    email: String::from("another@example.com"),
    ..user1  // 其余字段从 user1 获取（注意：会发生 move）
};

// 元组结构体
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);

// 类单元结构体（无字段）
struct AlwaysEqual;
let subject = AlwaysEqual;
```

## 方法语法
```rs
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 方法（第一个参数是 &self）
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 关联函数（无 self，用 :: 调用）
    fn square(size: u32) -> Self {
        Self { width: size, height: size }
    }
}

let rect = Rectangle { width: 30, height: 50 };
println!("面积: {}", rect.area());
let sq = Rectangle::square(3);
```

## 使用 #[derive(Debug)]
```rs
#[derive(Debug)]
struct Rectangle { width: u32, height: u32 }

let rect = Rectangle { width: 30, height: 50 };
println!("{:#?}", rect);  // 格式化打印
dbg!(&rect);              // 调试宏，打印文件和行号
```
