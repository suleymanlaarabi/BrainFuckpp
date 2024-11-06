use crate::{brain_pointer::BrainPointer, interprete};

#[derive(Clone)]
pub struct BrainFunk {
    pub ctx: BrainPointer,
    content: String,
}

impl BrainFunk {
    pub fn new(content: String) -> Self {
        let new_brain = BrainPointer::new();
        return BrainFunk {
            ctx: new_brain,
            content,
        };
    }
    pub fn init_params(&mut self, brain: &mut BrainPointer, params: Vec<i32>) {
        let old_cursor = brain.pointer_index;
        for (index, param) in params.iter().enumerate() {
            if *param == -1 {
                continue;
            }
            self.ctx.set_cursor(index);
            brain.set_cursor(*param as usize);
            self.ctx
                .set_cursor_value(brain.get_cursor_value().expect("Param not define").clone());
        }
        brain.set_cursor(old_cursor);
    }
    pub fn execute(&mut self, funcs: &mut Vec<BrainFunk>, brain: &mut BrainPointer) {
        let mut skip = 0;
        self.ctx.set_cursor(0);
        for (i, c) in self.content.chars().enumerate() {
            if skip > 0 {
                skip -= 1;
                continue;
            }
            skip = interprete(&self.content, c, &mut self.ctx, i, funcs);
        }
        brain.set_cursor_value(*self.ctx.get_cursor_value().unwrap());
    }
}
