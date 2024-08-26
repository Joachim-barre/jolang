use crate::generator::{Generate, IrGenerator};
use super::{Program, Statement};

impl Generate for Program {
    fn generate(&mut self, generator : &mut IrGenerator) {
        let blk = generator.append_block();
        generator.goto_begin(blk);
        for s in &mut self.0 {
            s.generate(generator);
        }
    }
}
