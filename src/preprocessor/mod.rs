mod if_statement;
mod include;

use include::parse_fetch_content;

fn fetch_translation(op: &str) -> String {
    match &op[1..4] {
        "inc" => parse_fetch_content(op),
        "if(" => "".to_string(),
        _ => "".to_string(),
    }
}

pub fn translate_bf(file_content: &str) -> String {
    let mut translated = String::new();
    let mut skip = 0;
    for (index, c) in file_content.chars().enumerate() {
        if skip > 0 {
            skip -= 1;
            continue;
        }
        match c {
            '!' => {
                let last_index = *&file_content[index + 1..]
                    .find("!")
                    .expect("preprocessor syntax not valid")
                    + 1;
                skip = last_index;
                translated.push_str(&fetch_translation(
                    &file_content[index..(index + last_index)],
                ));
            }
            _ => translated.push(c),
        }
    }
    return translated;
}
