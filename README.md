# Ctxs

此 crate 提供显式的上下文管理设施，它通常对 GUI 程序有用

# 示例

以下代码是对 `Mutex` 显式上下文的简易实现
```rust
use std::sync::Mutex;
impl<T> ContextMut<T> for Mutex<T> {
    fn context_mut<R, F: FnOnce(&mut T) -> R>(&self, local: F) -> R {
        let mut lock = self.lock().unwrap();
        local(&mut *lock)
    }
}
```

使用显式上下文
```rust
use std::sync::Mutex;
use ctxs::*;

static VALUE: Mutex<i32> = Mutex::new(0);

fn main() {
    VALUE.context_mut(|value| {
        *value += 1;
    });
    println!("Value: {}", VALUE.context_mut(|value| *value));
}
```