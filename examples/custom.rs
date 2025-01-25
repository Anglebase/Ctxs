use ctxs::*;

enum State {
    Enabled,
    Disabled,
}

struct Custom {
    state: State,
    value: i32,
}

impl MutContext<i32> for Custom {
    fn context<R, F: FnOnce(&mut i32) -> R>(&mut self, local: F) -> R {
        self.state = State::Enabled;
        let ret = local(&mut self.value);
        self.state = State::Disabled;
        ret
    }
}

fn main() {
    let mut custom = Custom {
        state: State::Disabled,
        value: 0,
    };

    let mut value = 10;

    custom.context(|v| {
        *v = value;
        value += 1;
    });

    println!("custom.value: {}", custom.value);
    println!("value: {}", value);
}