use std::fs::read_to_string;

pub fn parse_fetch_content(op: &str) -> String {
    let path_start_index = op.find("\"").expect("path not set");
    let path_end_index = &op[(path_start_index + 1)..]
        .find("\"")
        .expect("path syntaxe not valide");
    let path = &op[(path_start_index + 1)..(path_start_index + path_end_index + 1)];
    read_to_string(path).expect(&format!("Include path not valide: {}", path))
}
