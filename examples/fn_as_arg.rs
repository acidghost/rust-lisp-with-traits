use lisp::prelude::*;

fn run<A, R>(f: impl Fn(A) -> R, a: A) -> R {
    f(a)
}

fn hello(name: String) {
    println!("Hello, {}!", name);
}

fn main() {
    let name = "Sam".to_owned();
    let hello: fn(String) = hello;
    eval((run, hello, name));
}
