use std::env;

struct Options {}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut args_iter = args.iter();
    let mut options: Option<Options> = None;

    // Skip the target arg
    args_iter.next();

    if let Some(arg) = args_iter.next() {
        let ch = arg.chars().next().unwrap();

        if ch == '-' {
            options = Some(Options { });
        } else { println!("{arg}"); }
    }

    if let Some(options) = options {
        println!("Option found");
    }
}
