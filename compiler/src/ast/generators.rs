use crate::generator::{Generate, IrGenerator};
use super::Program;

impl Generate for Program {
    fn generate(&mut self, generator : &mut IrGenerator) {
        for s in &mut self.0 {
            s.generate();
        }
    }
}
