use std::env;

mod module;


const ARG_HELP: &str = "help";
const ARG_LOOP: &str = "loop";

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Error :: no args! Format ->  cargo run <module> <number>");
        return;
    }

    parsing_args(args);
}

fn parsing_args(args: Vec<String>) {
    let name = &args[1];

    match name.as_str() {
        ARG_HELP => {
            println!("----------- Service Modules Name ----------");
            println!("help");
            println!("loop , available number [1]");
            println!("----------- --------- ----------");
            println!("Example :  cargo run loop 1");
            println!("Example :  cargo run param 2");
            println!("----------- --------- ----------");
            return;
        }
        ARG_LOOP => {
            let (ok, number) = checking_module_number(args);
            if ok {
                match number {
                    1 => {
                        module::perulangan::loop1(1, 5);
                    }
                    _ => {
                        eprintln!("Unknown Module Number");
                    }
                }
            }
            return;
        }
        _ => {
            eprintln!("Unknown Arguments");
        }
    }

    fn checking_module_number(args: Vec<String>) -> (bool, i32) {
        if args.len() < 3 {
            eprintln!("Unknown Module Number");
            return (false, -1);
        }

        let result = args[2].parse::<i32>().unwrap_or(-1);
        if result == -1 {
            eprintln!("Invalid Module Number");
            return (false, -1);
        }

        return (true, result);
    }
}
