# 常见集合

Rust 标准库中的集合类型，数据存储在堆上，大小可以动态变化。

## Vector（动态数组）
```rs
// 创建
let v: Vec<i32> = Vec::new();
let v = vec![1, 2, 3];  // vec! 宏

// 修改
let mut v = Vec::new();
v.push(5);
v.push(6);

// 访问
let third: &i32 = &v[2];      // 越界会 panic
let third: Option<&i32> = v.get(2);  // 返回 Option，安全

// 遍历
for i in &v { println!("{i}"); }
for i in &mut v { *i += 50; }  // 解引用修改

// 使用枚举存储不同类型
enum SpreadsheetCell { Int(i32), Float(f64), Text(String) }
let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
];
```

## String（字符串）
```rs
let mut s = String::new();
let s = "initial contents".to_string();
let s = String::from("initial contents");

// 更新
let mut s = String::from("foo");
s.push_str("bar");  // 追加字符串切片
s.push('!');        // 追加字符

// 拼接
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2;    // s1 被 move，s2 是引用
let s = format!("{}-{}-{}", s1, s2, s3);  // 不会获取所有权

// 索引：Rust 字符串不支持索引 s[0]，因为 UTF-8 编码
// 遍历
for c in "Зд".chars() { println!("{c}"); }      // 字符
for b in "Зд".bytes() { println!("{b}"); }      // 字节
```

## HashMap（哈希映射）
```rs
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

// 访问
let team_name = String::from("Blue");
let score = scores.get(&team_name).copied().unwrap_or(0);

// 遍历
for (key, value) in &scores { println!("{key}: {value}"); }

// 只在键不存在时插入
scores.entry(String::from("Blue")).or_insert(50);

// 更新旧值（基于旧值）
let text = "hello world wonderful world";
let mut map = HashMap::new();
for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}
```
