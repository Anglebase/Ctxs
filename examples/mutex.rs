use ctxs::*;

use std::sync::Mutex;

static VALUE: Mutex<i32> = Mutex::new(0);

fn main() {
    VALUE.context_mut(|value| {
        *value += 1;
    });
    println!("Value: {}", VALUE.context_mut(|value| *value));
}
