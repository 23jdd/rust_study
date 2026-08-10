# Unsafe Rust 深入与实践

## 五种 unsafe 超能力

unsafe 块不会关闭 borrow checker，只是允许 5 种额外操作：

1. 解引用裸指针（raw pointer）
2. 调用 unsafe 函数/方法
3. 访问/修改可变静态变量
4. 实现 unsafe trait
5. 访问 union 字段

## 1. 裸指针（Raw Pointer）

```rs
// *const T —— 不可变裸指针
// *mut T  —— 可变裸指针

let mut num = 5;
let r1 = &num as *const i32;     // 从引用创建
let r2 = &mut num as *mut i32;   // 从可变引用创建
let r3 = 0x1000 as *const i32;   // 从地址创建（危险！）

unsafe {
    println!("r1: {}", *r1);
    *r2 += 1;
}
```

**裸指针与引用的区别：**
- 允许忽略借用规则（可同时有不可变和可变指针）
- 不保证指向有效内存
- 允许为空
- 不实现自动清理

## 2. 调用 unsafe 函数

```rs
unsafe fn dangerous() {
    // 这个函数自身就是 unsafe 的
}

unsafe {
    dangerous();
}
```

### 标准库中的 unsafe 函数示例

```rs
use std::slice;

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
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

let mut v = vec![1, 2, 3, 4, 5, 6];
let (left, right) = split_at_mut(&mut v, 3);
```

### FFI（Foreign Function Interface）调用 C 代码

```rs
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("abs(-3) = {}", abs(-3));
    }
}

// 从 C 调用 Rust
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Called from C!");
}
```

## 3. 可变静态变量

```rs
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    add_to_count(3);
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}
// 多线程下访问 static mut 可能导致数据竞争
```

## 4. unsafe trait

```rs
unsafe trait Foo {
    fn foo(&self);
}

unsafe impl Foo for i32 {
    fn foo(&self) { println!("{}", self); }
}
// 实现者必须保证 trait 的安全约定
```

### Send 和 Sync 的手动实现

```rs
use std::cell::RefCell;
use std::rc::Rc;

// Rc<T> 不是 Send，但我们知道只用在一个线程中时可以手动实现
struct MyBox<T> {
    inner: Rc<RefCell<T>>,
}

// 不安全：我们承诺此类型只在单线程使用
unsafe impl<T> Send for MyBox<T> {}
unsafe impl<T> Sync for MyBox<T> {}
```

## 5. Union

```rs
union MyUnion {
    f1: u32,
    f2: f32,
}

let u = MyUnion { f1: 1 };
unsafe {
    println!("u.f1 = {}", u.f1); // 1
    println!("u.f2 = {}", u.f2); // 未定义行为（可能打印垃圾值）
}
```

---

# 练习题

## 练习 1：实现自定义裸指针切片

编写一个函数，接收一个 `&[i32]`，用裸指针返回切片中所有元素的和。

```rs
fn sum_slice_raw(slice: &[i32]) -> i32 {
    // TODO: 用裸指针实现
}
```

<details>
<summary>参考解答</summary>

```rs
fn sum_slice_raw(slice: &[i32]) -> i32 {
    let ptr = slice.as_ptr();
    let len = slice.len();
    let mut sum = 0;
    for i in 0..len {
        unsafe {
            sum += *ptr.add(i);
        }
    }
    sum
}
```
</details>

## 练习 2：实现安全的 split_at_mut

用 unsafe 实现 `Vec<T>` 的 `split_at_mut` 方法。

```rs
fn my_split_at_mut<T>(slice: &mut [T], mid: usize) -> (&mut [T], &mut [T]) {
    // TODO
}
```

<details>
<summary>参考解答</summary>

```rs
use std::slice;

fn my_split_at_mut<T>(slice: &mut [T], mid: usize) -> (&mut [T], &mut [T]) {
    let len = slice.len();
    assert!(mid <= len);
    let ptr = slice.as_mut_ptr();
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
```
</details>

## 练习 3：实现自定义 Vec<T>（简化版）

用裸指针和 unsafe 实现一个简化版的 `Vec<T>`，支持 `new`、`push`、`pop`、`len`、`get` 和 `Drop`。

```rs
struct MyVec<T> {
    ptr: *mut T,
    len: usize,
    cap: usize,
}

impl<T> MyVec<T> {
    fn new() -> Self {
        // TODO
        todo!()
    }

    fn push(&mut self, value: T) {
        // TODO: 需要处理扩容
        todo!()
    }

    fn pop(&mut self) -> Option<T> {
        // TODO
        todo!()
    }

    fn len(&self) -> usize {
        // TODO
        todo!()
    }

    fn get(&self, index: usize) -> Option<&T> {
        // TODO
        todo!()
    }
}

impl<T> Drop for MyVec<T> {
    fn drop(&mut self) {
        // TODO
        todo!()
    }
}
```

<details>
<summary>参考解答</summary>

```rs
use std::alloc::{self, Layout};
use std::ptr::NonNull;

struct MyVec<T> {
    ptr: NonNull<T>,
    len: usize,
    cap: usize,
}

impl<T> MyVec<T> {
    fn new() -> Self {
        MyVec {
            ptr: NonNull::dangling(),
            len: 0,
            cap: 0,
        }
    }

    fn push(&mut self, value: T) {
        if self.len == self.cap {
            self.grow();
        }
        unsafe {
            self.ptr.as_ptr().add(self.len).write(value);
        }
        self.len += 1;
    }

    fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            unsafe { Some(self.ptr.as_ptr().add(self.len).read()) }
        }
    }

    fn len(&self) -> usize { self.len }

    fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            None
        } else {
            unsafe { Some(&*self.ptr.as_ptr().add(index)) }
        }
    }

    fn grow(&mut self) {
        let (new_cap, new_layout) = if self.cap == 0 {
            (4, Layout::array::<T>(4).unwrap())
        } else {
            let new_cap = self.cap * 2;
            (new_cap, Layout::array::<T>(new_cap).unwrap())
        };

        let new_ptr = if self.cap == 0 {
            unsafe { NonNull::new(alloc::alloc(new_layout) as *mut T).unwrap() }
        } else {
            let old_layout = Layout::array::<T>(self.cap).unwrap();
            let old_ptr = self.ptr.as_ptr() as *mut u8;
            unsafe {
                let new_ptr = alloc::realloc(old_ptr, old_layout, new_layout.size());
                NonNull::new(new_ptr as *mut T).unwrap()
            }
        };

        self.ptr = new_ptr;
        self.cap = new_cap;
    }
}

impl<T> Drop for MyVec<T> {
    fn drop(&mut self) {
        if self.cap == 0 {
            return;
        }
        // 先 drop 所有元素
        for i in 0..self.len {
            unsafe { self.ptr.as_ptr().add(i).drop_in_place(); }
        }
        // 再释放内存
        let layout = Layout::array::<T>(self.cap).unwrap();
        unsafe { alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout); }
    }
}
```
</details>

## 练习 4：FFI 调用

调用 C 标准库的 `strlen` 函数计算字符串长度。

```rs
// TODO: 声明外部 C 函数
// TODO: 在 unsafe 块中调用
```

<details>
<summary>参考解答</summary>

```rs
use std::ffi::CString;

extern "C" {
    fn strlen(s: *const i8) -> usize;
}

fn my_strlen(s: &str) -> usize {
    let c_str = CString::new(s).unwrap();
    unsafe { strlen(c_str.as_ptr()) }
}

fn main() {
    let len = my_strlen("hello");
    println!("length = {}", len); // 5
}
```
</details>

## 练习 5：安全包装 unsafe —— 实现安全的链表

用 unsafe 实现一个单向链表，但对外暴露完全安全的 API。

```rs
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    len: usize,
}

impl<T> LinkedList<T> {
    fn new() -> Self { ... }
    fn push_front(&mut self, value: T) { ... }
    fn pop_front(&mut self) -> Option<T> { ... }
    fn len(&self) -> usize { ... }
    // 用 unsafe 实现一个返回可变迭代器的方法
    fn iter_mut(&mut self) -> IterMut<T> { ... }
}
```

<details>
<summary>参考解答</summary>

```rs
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    len: usize,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None, len: 0 }
    }

    pub fn push_front(&mut self, value: T) {
        let new_node = Box::new(Node {
            value,
            next: self.head.take(),
        });
        self.head = Some(new_node);
        self.len += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.len -= 1;
            node.value
        })
    }

    pub fn len(&self) -> usize { self.len }

    // 可变迭代器需要 unsafe（安全包装）
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            next: self.head.as_deref_mut().map(|node| &mut *node as *mut Node<T>),
            _marker: std::marker::PhantomData,
        }
    }
}

pub struct IterMut<'a, T> {
    next: Option<*mut Node<T>>,
    _marker: std::marker::PhantomData<&'a mut T>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|ptr| {
            let node = unsafe { &mut *ptr };
            self.next = node.next.as_deref_mut().map(|n| n as *mut Node<T>);
            &mut node.value
        })
    }
}
```
</details>

## 练习 6：为什么要 unsafe —— 判别题目

判断以下场景是否需要 unsafe，并说明原因：

1. 修改 `&mut T` 指向的值
2. 让两个可变引用同时指向同一块内存
3. 将一个 `i32` 指针转换为 `f32` 指针并读取
4. 在多线程中共享 `Rc<T>`
5. 从 `&[u8]` 构造 `&str`（已知是有效 UTF-8）

<details>
<summary>参考解答</summary>

| 场景 | 需要 unsafe? | 原因 |
|------|-------------|------|
| 1 | ❌ 不需要 | `&mut` 本身就可以修改值 |
| 2 | ✅ 需要 | 违反借用规则，需要用裸指针绕过 |
| 3 | ✅ 需要 | 类型 punning 是 UB，只能在 union 或 transmute 中做 |
| 4 | ✅ 需要 | Rc 不是 Send | Sync，需要 unsafe 手动实现或改用 Arc |
| 5 | ✅ 需要 | `str::from_utf8_unchecked` 是 unsafe 的 |

</details>
