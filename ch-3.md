# 常见编程概念
## 变量和可变性
### 变量默认是不可变的（immutable）
### example
```rs
  let a=1;
  a=2; // error
  let mut a=1;  // varbible shadow 我们可以定义一个与之前变量同名的新变量
  a=2; // true 
  const Max:i32=10000_000  //  编译时常量
```
## 数据类型
### 标量类型
|长度	|有符号|无符号
|-----|-----|----|
|8-bit|	i8|	u8
|16-bit|	i16|	u16
|32-bit|	i32	|u32
|64-bit	|i64	|u64
|128-bit	|i128	|u128
|架构相关	|isize|	usize

- bool true or false
- char  4 byte  'a' 'b' 'c'


|数字字面值	|例子|
|-------|-----|
|Decimal（十进制）|	98_222
|Hex（十六进制）|	0xff
|Octal（八进制 |	0o77
|Binary（二进制）|	0b1111_0000
|Byte（字节字面值，仅限 u8）|	b'A'

## 复合类型
### array
```rs
 let a=[1,2,3,4]  //  [i32;4]
 let a=[1;5]  // a=[1,1,1,1,1]
```
####  元组类型
```rs
let a=(1,2,3) // (i32,i32,i32)
```

## 函数
```rs
//   a,b is input args  i32 is output 
fn add(a:i32,b:i32)->i32{
     return a+b;  // or a+b     
}
```

## 控制流
### if
```rs
let a=1;
if a==1 || a==2{
      
} else if{
     
} else{
     
}

```
###  while
```rs
let mut sum=0;
let count=0;
while count<100{
      sum+=count;
      count++;   
}
```

## for
```rs
// for i in iteor
for i in 1..=100{
     
}
```
####  loop

```rs

loop{
     
}
```
#### break,continue
```rs
use std::thread::sleep;
'a:loop{
   sleep(sleep(std::time::Duration::new(1,0)))
   break 'a     
}
let a=loop{ 
      break 2
}
```