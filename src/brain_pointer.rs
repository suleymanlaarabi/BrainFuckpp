use std::fmt::Display;

use crate::custom_impl::Magnitude;

#[derive(Clone)]
pub struct BrainPointer {
    pub pointer_list: Vec<u8>,
    pub pointer_index: usize,
}

impl BrainPointer {
    pub fn get_cursor_mut_value(&mut self) -> Option<&mut u8> {
        self.pointer_list.get_mut(self.pointer_index)
    }
    pub fn get_cursor_value(&self) -> Option<&u8> {
        self.pointer_list.get(self.pointer_index)
    }
    pub fn incr_cursor_value(&mut self) {
        if let Some(value) = self.get_cursor_mut_value() {
            *value += 1;
        }
    }
    pub fn decr_cursor_value(&mut self) {
        if let Some(value) = self.get_cursor_mut_value() {
            *value -= 1;
        }
    }
    pub fn incr_cursor(&mut self) {
        self.pointer_index += 1;
        if self.pointer_index >= self.pointer_list.len() {
            self.pointer_list.push(0);
        }
    }
    pub fn set_cursor(&mut self, index: usize) {
        self.pointer_index = index;
        if self.pointer_index >= self.pointer_list.len() {
            self.pointer_list.push(0);
        }
    }
    pub fn set_cursor_value(&mut self, value: u8) {
        if let Some(pvalue) = self.get_cursor_mut_value() {
            *pvalue = value;
        }
    }
    pub fn decr_cursor(&mut self) {
        self.pointer_index -= 1;
    }
    pub fn print_value(&self) {
        print!("{}", (*self.get_cursor_value().unwrap()) as char);
    }
    pub fn new() -> Self {
        let mut brain = BrainPointer {
            pointer_list: Vec::new(),
            pointer_index: 0,
        };
        brain.pointer_list.push(0);
        return brain;
    }
}

impl Display for BrainPointer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = String::new();
        for (index, c) in self.pointer_list.iter().enumerate() {
            str.push_str(&format!("  {}", index));
            if index < 10 {
                for _ in 0..c.magnitude() {
                    str.push(' ');
                }
            } else {
                for _ in 1..c.magnitude() {
                    str.push(' ');
                }
            }
        }
        str.push('\n');
        self.pointer_list.iter().for_each(|c| {
            str.push_str(&format!("| {} ", *c));
        });
        str.push_str("|\n");
        for (index, c) in self.pointer_list.iter().enumerate() {
            if self.pointer_index == index {
                str.push_str("  ^")
            } else {
                str.push_str("   ")
            }
            for _ in 0..c.magnitude() {
                str.push(' ');
            }
        }
        writeln!(f, "{}", str)
    }
}
