use std::env;

use entity::user::User as user_entity;

mod module;
mod entity;

const ARG_HELP: &str = "help";
const ARG_LOOP: &str = "loop";
const ARG_TUPLES: &str = "tuples";
const ARG_CLOSURES: &str = "closures";
const ARG_STRUCT: &str = "struct";
const ARG_GENERIC: &str = "generic";

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
            action_module_help();
        }
        ARG_LOOP => {
            action_module_loop(args);
        }
        ARG_TUPLES => {
            action_module_tuples(args);
        }
        ARG_CLOSURES => {
            action_module_closures(args);
        }
        ARG_STRUCT => {
            action_module_struct(args);
        }
        ARG_GENERIC=>{
            action_module_generic(args);
        }
        _ => {
            eprintln!("Unknown Arguments");
        }
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

fn action_module_help() {
    println!("----------- Service Modules Name ----------");
    println!("help");
    println!("loop , available number [1]");
    println!("----------- --------- ----------");
    println!("Example :  cargo run loop 1");
    println!("Example :  cargo run param 2");
    println!("----------- --------- ----------");
}

fn action_module_loop(args: Vec<String>) {
    let (ok, number) = checking_module_number(args);
    if ok {
        match number {
            1 => module::perulangan::loop1(1, 5),
            2 => module::perulangan::loop2(),
            3 => module::perulangan::loop3(),
            _ => {
                eprintln!("Unknown Module Number");
            }
        }
    }
}

fn action_module_tuples(args: Vec<String>) {
    let (ok, number) = checking_module_number(args);
    if ok {
        match number {
            1 => module::tuples::tuples1(),
            2 => module::tuples::tuples2(),
            _ => {
                eprintln!("Unknown Module Number");
            }
        }
    }
}

fn action_module_closures(args: Vec<String>) {
    let (ok, number) = checking_module_number(args);
    if ok {
        match number {
            1 => module::closures::closures1(),
            2 => module::closures::closures2(),
            3 => module::closures::closures3(),
            _ => {
                eprintln!("Unknown Module Number");
            }
        }
    }
}

fn action_module_struct(args: Vec<String>) {
    let (ok, number) = checking_module_number(args);
    if ok {
        match number {
            1 => {
                let data = user_entity { name: "OpannapO".to_string(), age: 33 };
                module::structs::structs1(data);
            }
            2 => {
                let data = user_entity { name: "Taufan Alfazri Baru Belajar".to_string(), age: 33 };
                module::structs::structs2(data);
            }
            _ => {
                eprintln!("Unknown Module Number");
            }
        }
    }
}

fn action_module_generic(args: Vec<String>) {
    let (ok, number) = checking_module_number(args);
    if ok {
        match number {
            1 => {
                module::generic::generic1();
            }
            _ => {
                eprintln!("Unknown Module Number");
            }
        }
    }

}