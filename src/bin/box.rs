use std::alloc::{self, Layout};
use std::fmt;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;

/// 一个简化版的 `Box<T>`：拥有堆上分配的单个 T 的所有权
pub struct MyBox<T: ?Sized> {
    ptr: NonNull<T>,
    _marker: PhantomData<T>,
}

// Box<T> 可以 Send 当且仅当 T: Send（拥有权转移）
unsafe impl<T: ?Sized + Send> Send for MyBox<T> {}
// Box<T> 可以 Sync 当且仅当 T: Sync（通过 &Box<T> 访问内部数据）
unsafe impl<T: ?Sized + Sync> Sync for MyBox<T> {}

impl<T> MyBox<T> {
    /// 在堆上分配内存并放入值
    pub fn new(x: T) -> Self {
        // 处理 ZST：零大小类型不需要真实分配
        if std::mem::size_of::<T>() == 0 {
            return Self {
                ptr: NonNull::dangling(),
                _marker: PhantomData,
            };
        }

        let layout = Layout::new::<T>();
        let ptr = unsafe { alloc::alloc(layout) };

        if ptr.is_null() {
            alloc::handle_alloc_error(layout);
        }

        // 将裸指针转为 T 的指针，写入值
        let ptr = ptr as *mut T;
        unsafe {
            ptr.write(x);
        }

        Self {
            ptr: unsafe { NonNull::new_unchecked(ptr) },
            _marker: PhantomData,
        }
    }

    /// 从裸指针构造 Box（unsafe：调用者必须保证 ptr 是唯一的堆分配）
    /// # Safety
    /// - ptr 必须是通过 GlobalAlloc 分配的、有效的、未别名的 *mut T
    /// - ptr 必须指向已初始化的 T
    pub unsafe fn from_raw(ptr: *mut T) -> Self {
        Self {
            ptr: NonNull::new(ptr).expect("from_raw called with null pointer"),
            _marker: PhantomData,
        }
    }

    /// 消费 Box，返回裸指针，不再自动释放
    pub fn into_raw(b: Self) -> *mut T {
        let ptr = b.ptr.as_ptr();
        // 忘记 self，防止 Drop 时释放内存
        std::mem::forget(b);
        ptr
    }

    /// 获取底层指针（不消费 Box）
    pub fn as_ptr(b: &Self) -> *mut T {
        b.ptr.as_ptr()
    }
}

impl<T: ?Sized> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        // SAFETY: Box 拥有这块内存的唯一所有权，且 ptr 始终有效
        unsafe { self.ptr.as_ref() }
    }
}

impl<T: ?Sized> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut T {
        // SAFETY: Box 拥有唯一所有权，&mut self 保证独占访问
        unsafe { self.ptr.as_mut() }
    }
}

impl<T: ?Sized> Drop for MyBox<T> {
    fn drop(&mut self) {
        // ZST 没有真实分配，不需要 dealloc
        if std::mem::size_of_val(unsafe { self.ptr.as_ref() }) == 0 {
            return;
        }

        // SAFETY: ptr 来自 alloc，且我们拥有唯一所有权
        unsafe {
            // 先调用析构函数
            std::ptr::drop_in_place(self.ptr.as_ptr());
            // 再释放内存
            let layout = Layout::for_value(self.ptr.as_ref());
            alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout);
        }
    }
}

// ===== 让 Box 更好用：标准 trait 实现 =====

impl<T: ?Sized + fmt::Debug> fmt::Debug for MyBox<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

impl<T: ?Sized + fmt::Display> fmt::Display for MyBox<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&**self, f)
    }
}

impl<T: Default> Default for MyBox<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T> From<T> for MyBox<T> {
    fn from(x: T) -> Self {
        Self::new(x)
    }
}

// 支持部分移动（partial move）
impl<T: Clone> Clone for MyBox<T> {
    fn clone(&self) -> Self {
        Self::new((**self).clone())
    }
}

impl<T: ?Sized + PartialEq> PartialEq for MyBox<T> {
    fn eq(&self, other: &Self) -> bool {
        PartialEq::eq(&**self, &**other)
    }
}

// ===== 测试 =====

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_usage() {
        let b = MyBox::new(42);
        assert_eq!(*b, 42);
    }

    #[test]
    fn mutability() {
        let mut b = MyBox::new(String::from("hello"));
        b.push_str(" world");
        assert_eq!(&*b, "hello world");
    }

    #[test]
    fn zst() {
        struct ZeroSized;
        let b = MyBox::new(ZeroSized);
        let _ = *b; // 不会 panic
    }

    #[test]
    fn drop_runs() {
        use std::sync::atomic::{AtomicUsize, Ordering};
        static DROP_COUNT: AtomicUsize = AtomicUsize::new(0);

        struct CountDrop;
        impl Drop for CountDrop {
            fn drop(&mut self) {
                DROP_COUNT.fetch_add(1, Ordering::SeqCst);
            }
        }

        {
            let _b = MyBox::new(CountDrop);
        }
        assert_eq!(DROP_COUNT.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn into_raw_and_from_raw() {
        let b = MyBox::new(100);
        let raw = MyBox::into_raw(b);
        unsafe {
            assert_eq!(*raw, 100);
            let b = MyBox::from_raw(raw);
            drop(b); // 安全释放
        }
    }
}

fn main() {
    // 演示
    let mut b = MyBox::new(vec![1, 2, 3]);
    b.push(4);
    println!("{:?}", b); // [1, 2, 3, 4]

    let zst = MyBox::new(());
    drop(zst); // ZST 安全释放
}