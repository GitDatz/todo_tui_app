use std::env;

mod constants;
mod data;
mod ui;
mod presenter;
mod types;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        presenter::presenter(&args[1]);
    }
    else {
        presenter::presenter("");
    }
}