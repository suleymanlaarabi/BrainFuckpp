use std::{
    env,
    fs::read_to_string,
    io::{self, Read},
};

mod parser;
use brain_func::BrainFunk;
use brain_pointer::BrainPointer;
use parser::{fetch_func, parse_function_params};
use preprocessor::translate_bf;
mod brain_func;
mod brain_pointer;
mod custom_impl;
mod preprocessor;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_content = translate_bf(
        &read_to_string(args.get(1).expect("no file set")).expect("file reading error"),
    );
    let mut brain = BrainPointer::new();
    let mut funcs = fetch_func(&file_content);
    let mut skip = 0;

    for (i, c) in file_content.chars().enumerate() {
        if skip > 0 {
            skip -= 1;
            continue;
        }
        skip = interprete(&file_content, c, &mut brain, i, &mut funcs);
    }
}

fn interprete(
    file_content: &str,
    c: char,
    brain: &mut BrainPointer,
    index: usize,
    funcs: &mut Vec<BrainFunk>,
) -> i32 {
    match c {
        '>' => brain.incr_cursor(),
        '<' => brain.decr_cursor(),
        '+' => brain.incr_cursor_value(),
        '-' => brain.decr_cursor_value(),
        '.' => brain.print_value(),
        ',' => {
            let mut buf: [u8; 1] = [0];
            let _ = io::stdin().read(&mut buf).unwrap();
            brain.set_cursor_value(buf[0]);
        }
        '[' => {
            return interprete_loop(&file_content[(index + 1)..], brain, funcs);
        }
        '|' => {
            let next_char = *&file_content[index + 1..].chars().next().unwrap();
            if next_char != '~' {
                return 0;
            }
            let skip = &file_content[index + 1..]
                .find("~~|")
                .expect("function syntax not valid");
            return *skip as i32;
        }
        '@' => {
            let (params, end_index) = parse_function_params(index, file_content, brain);
            let mut func = funcs
                .get_mut(*brain.get_cursor_value().expect("not define") as usize)
                .expect("function not defined")
                .clone();
            func.init_params(brain, params);
            func.execute(funcs, brain);
            return end_index;
        }
        '#' => print!("{}", brain),
        _ => {
            return 0;
        }
    }

    return 0;
}

fn interprete_loop(str: &str, brain: &mut BrainPointer, funcs: &mut Vec<BrainFunk>) -> i32 {
    for (i, c) in str.chars().enumerate() {
        if c == ']' && *brain.get_cursor_value().unwrap() <= 0 {
            return i as i32;
        } else if c == ']' {
            return interprete_loop(str, brain, funcs);
        }
        interprete(str, c, brain, i, funcs);
    }
    return 0;
}
