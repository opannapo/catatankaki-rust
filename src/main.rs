use std::env;

const ARG_HELP: &str = "help";

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Error :: no args! Format :  cargo run <module> <number>");
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
            println!("----------- --------- ----------");
            println!("Example :  cargo run loop 1");
            println!("Example :  cargo run param 2");
            println!("----------- --------- ----------");
            return;
        }
        _ => {
            println!("Unknown Arguments")
        }
    }
}
