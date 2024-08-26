use jolang_shared::ir::instructions::Instruction;
use crate::scope::{Scope, ScopeKind};
use crate::generator::{Generate, IrGenerator};
use super::{Expr, Program, Statement};

impl Generate for Program {
    fn generate(&self, generator : &mut IrGenerator) {
        let scope = Scope::new(ScopeKind::Root);
        generator.enter_scope(scope);
        let blk = generator.append_block();
        generator.goto_begin(blk);
        for s in &self.0 {
            s.generate(generator);
        }
        generator.exit_scope();
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
            Self::If(expr, then, _else) => {
                expr.generate(generator);
                let cond = generator.get_current_block().unwrap().last_index();
                let then_block = generator.append_block();
                let else_block = generator.append_block();
                let after_block = match _else {
                    Some(_) => generator.append_block(),
                    None => else_block
                };
                generator.add(Instruction::Briz(then_block, else_block, cond));
                generator.goto_begin(then_block);
                match **then {
                    Self::Block(_) => then.generate(generator),
                    _ => {
                        let scope = Scope::new(ScopeKind::Block);
                        generator.enter_scope(scope);
                        then.generate(generator);
                        generator.exit_scope();
                    }
                }
                generator.add(Instruction::Br(after_block));
                generator.goto_begin(else_block);
                if let Some(_else) = _else {
                    match **_else {
                        Self::Block(_) => then.generate(generator),
                        _ => {
                            let scope = Scope::new(ScopeKind::Block);
                            generator.enter_scope(scope);
                            _else.generate(generator);
                            generator.exit_scope();
                        }
                    }
                    generator.add(Instruction::Br(after_block));
                    generator.goto_begin(after_block);
                }
            },
            _ => todo!()
        }
    }
}

impl Generate for Expr {
    fn generate(&self, generator : &mut IrGenerator) {
        todo!()
    }
}
