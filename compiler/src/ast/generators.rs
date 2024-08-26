use crate::scope::{Scope, ScopeKind};
use crate::generator::{Generate, IrGenerator};
use super::{Program, Statement};

impl Generate for Program {
    fn generate(&self, generator : &mut IrGenerator) {
        let blk = generator.append_block();
        generator.goto_begin(blk);
        for s in &self.0 {
            s.generate(generator);
        }
    }
}

impl Generate for Statement {
    fn generate(&self, generator : &mut IrGenerator) {
        match self {
            Self::Block(stmts) => {
                let scope = Scope::new(ScopeKind::Block);
                generator.enter_scope(scope);
                for s in stmts {
                    s.generate(generator);
                }
                generator.exit_scope();
            },
            _ => todo!()
        }
    }
}
