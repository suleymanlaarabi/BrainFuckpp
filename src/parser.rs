use crate::{brain_func::BrainFunk, brain_pointer::BrainPointer};

pub fn fetch_func(file_content: &String) -> Vec<BrainFunk> {
    let mut brains_func = Vec::new();
    let mut finded = 0;
    file_content.split("~~").for_each(|str| {
        finded += 1;
        if finded == 2 {
            finded = 0;
            brains_func.push(BrainFunk::new(str.to_owned()));
        }
    });
    brains_func
}

pub fn parse_function_params(
    index: usize,
    file_content: &str,
    brain: &BrainPointer,
) -> (Vec<i32>, i32) {
    let start_index = index;
    let end_index = &file_content[index..]
        .find(')')
        .expect("Function syntaxe not correct");
    let content = &file_content[(start_index + 2)..(index + end_index)].to_owned();
    let params: Vec<i32> = content
        .split(",")
        .map(|str| {
            if str.trim().len() < 1 {
                return -1;
            }
            let mut cursor = 0;
            for op in str.chars() {
                match op {
                    '>' => cursor += 1,
                    '<' => cursor -= 1,
                    _ => {}
                }
            }
            return cursor + brain.pointer_index as i32;
        })
        .collect();
    return (params, *end_index as i32);
}
