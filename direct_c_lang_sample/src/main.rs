use direct_c_lang::c_lang;
use std::{arch::global_asm, ffi::c_int};

extern "C" {
    fn fib(n: c_int) -> c_int;
}

fn main() {
    unsafe { fib(3) };
    println!("Hello, world! from Rust.");
    println!(
        "{:?}",
        (0..10)
            .into_iter()
            .map(|n| unsafe { fib(n) })
            .collect::<Vec<_>>()
    );
}

c_lang! {
    int fib(int n) {
        if (n < -1) {
            return -1;
        }
        if (n == 0 || n == 1) {
            return 1;
        }

        return fib(n - 1) + fib(n - 2);
    }
}
