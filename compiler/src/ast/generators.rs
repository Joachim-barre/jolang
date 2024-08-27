use jolang_shared::ffi::jolang_std::JOLANG_STD;
use jolang_shared::ir::instructions::Instruction;
use crate::scope::{Scope, ScopeKind};
use crate::generator::{self, Generate, IrGenerator};
use super::{Call, Expr, Ident, PrimaryExpr, Program, Statement, UnaryOp};

impl Generate for Program {
    fn generate(&self, generator : &mut IrGenerator) {
        let blk = generator.append_block();
        let exit_block = generator.append_block();
        generator.goto_begin(exit_block);
        generator.add(Instruction::Iconst(0)).unwrap();
        generator.add(Instruction::Reti());
        let scope = Scope::new(ScopeKind::Root, blk, exit_block);
        generator.enter_scope(scope);
        generator.goto_begin(blk);
        for s in &self.0 {
            s.generate(generator);
        }
        generator.exit_scope();
        generator.add(Instruction::Br(exit_block));
    }
}

impl Generate for Statement {
    fn generate(&self, generator : &mut IrGenerator) {
        match self {
            Self::Block(stmts) => {
                let block = generator.append_block();
                generator.pass_vars();
                generator.add(Instruction::Br(block));
                generator.goto_begin(block);
                generator.recive_vars();
                let after_block = generator.append_block();
                let scope = Scope::new(ScopeKind::Block, block, after_block);
                generator.enter_scope(scope);
                for s in stmts {
                    s.generate(generator);
                }
                generator.exit_scope();
                generator.pass_vars();
                generator.add(Instruction::Br(after_block));
                generator.goto_begin(after_block);
                generator.recive_vars();
            },
            Self::If(expr, then, _else) => {
                expr.generate(generator);
                let cond = generator.stack_size().unwrap() -1;
                let then_block = generator.append_block();
                let else_block = generator.append_block();
                let after_block = match _else {
                    Some(_) => generator.append_block(),
                    None => else_block
                };
                generator.pass_vars();
                generator.add(Instruction::Dupx(cond as i64));
                generator.add(Instruction::Briz(else_block, then_block));
                generator.goto_begin(then_block);
                generator.recive_vars();
                
                let scope = Scope::new(ScopeKind::Block, then_block, after_block);
                generator.enter_scope(scope);
                then.generate(generator);
                generator.exit_scope();
                generator.pass_vars();
                generator.add(Instruction::Br(after_block));
                
                generator.goto_end(else_block);
                if let Some(_else) = _else {
                    generator.pass_vars();
                    let scope = Scope::new(ScopeKind::Block, else_block, after_block);
                    generator.enter_scope(scope);
                    _else.generate(generator);
                    generator.exit_scope();
                    generator.pass_vars();
                    generator.add(Instruction::Br(after_block));
                    generator.goto_begin(after_block);
                }
                generator.recive_vars();
            },
            Statement::While(expr, body) => {
                let while_cond = generator.append_block();
                generator.pass_vars();
                generator.add(Instruction::Br(while_cond));
                generator.goto_begin(while_cond);
                generator.recive_vars();
                let while_body = generator.append_block();
                let after_block = generator.append_block();
                let scope = Scope::new(ScopeKind::Loop, while_cond, after_block);
                generator.enter_scope(scope);
                expr.generate(generator);
                let cond = generator.stack_size().unwrap() -1;
                generator.pass_vars();
                generator.add(Instruction::Dupx(cond as i64));
                generator.add(Instruction::Briz(after_block, while_body));
                generator.goto_begin(while_body);
                generator.recive_vars();
                body.generate(generator);
                generator.add(Instruction::Br(while_cond));
                generator.exit_scope();
                generator.goto_begin(after_block);
                generator.recive_vars();
            },
            Self::Loop(body) => {
                let loop_body = generator.append_block();
                let after_block = generator.append_block();
                generator.pass_vars();
                generator.add(Instruction::Br(loop_body));
                generator.goto_begin(loop_body);
                generator.recive_vars();
                let scope = Scope::new(ScopeKind::Loop, loop_body, after_block);
                generator.enter_scope(scope);                
                body.generate(generator);
                generator.exit_scope();
                generator.pass_vars();
                generator.add(Instruction::Br(loop_body));
                generator.goto_begin(after_block);
                generator.recive_vars();
            },
            Self::Return(expr) => {
                expr.generate(generator);
                generator.add(Instruction::Reti());
            },
            Self::Continue => {
                generator.add(Instruction::Br(generator.get_scopes()
                    .iter()
                    .filter(|x| x.kind == ScopeKind::Loop)
                    .next()
                    .expect("can't continue outside a loop")
                    .block));
            },
            Self::Break => {
                generator.add(Instruction::Br(generator.get_scopes()
                    .iter()
                    .filter(|x| x.kind == ScopeKind::Loop)
                    .next()
                    .expect("can't continue outside a loop")
                    .exit));
            },
            Self::VarDecl(name, value) => {
                let default_value = match value {
                    Some(Expr::PrimaryExpr(p)) => match p {
                        super::PrimaryExpr::Litteral(v) => v.clone(),
                        _ => 0
                    },
                    _ => 0
                };
                let id = generator.decl_var(name.to_string(), default_value);
                if default_value == 0 && value.is_some() {
                    value.as_ref().unwrap().generate(generator);
                    let val = generator.get_current_block().unwrap().last_index();
                    generator.add(Instruction::Varset(id, val));
                }
            },
            Self::VarSet(name, value) => {
                value.generate(generator);
                let val = generator.get_current_block().unwrap().last_index();
                let id = generator.get_varid(name.to_string()).expect(format!("unknown variable : {}", name).as_str());
                generator.add(Instruction::Varset(id, val));
            },
            Self::Call(call) => {
                call.generate(generator);
            }
        }
    }
}

impl Generate for Expr {
    fn generate(&self, generator : &mut IrGenerator) {
        match self {
            Expr::PrimaryExpr(p) => p.generate(generator),
            Expr::UnaryExpr(op, p) => {
                p.generate(generator);
                match op {
                    UnaryOp::Plus => (),
                    UnaryOp::Minus => {
                        let val = generator.get_current_block().unwrap().last_index();
                        generator.add(Instruction::Neg(val));
                        ()
                    }
                };
            },
            Expr::BinExpr(e1, e2, op) => {
                e1.generate(generator);
                let val1 = generator.get_current_block().unwrap().last_index();
                e2.generate(generator);
                let val2 = generator.get_current_block().unwrap().last_index();
                match op {
                    super::BinOp::Add => {
                        generator.add(Instruction::Add(val1, val2))
                    },
                    super::BinOp::Sub => {
                        generator.add(Instruction::Sub(val1, val2))
                    },
                    super::BinOp::Mul => {
                        generator.add(Instruction::Mul(val1, val2))
                    },
                    super::BinOp::Div => {
                        generator.add(Instruction::Div(val1, val2))
                    },
                    super::BinOp::Equal => {
                        generator.add(Instruction::Eq(val1, val2))
                    },
                    super::BinOp::NotEqual => {
                        generator.add(Instruction::Ne(val1, val2))
                    },
                    super::BinOp::Greater => {
                        generator.add(Instruction::Gt(val1, val2))
                    },
                    super::BinOp::GreaterEqual => {
                        generator.add(Instruction::Ge(val1, val2))
                    },
                    super::BinOp::LesserEqual => {
                        generator.add(Instruction::Le(val1, val2))
                    },
                    super::BinOp::Lesser => {
                        generator.add(Instruction::Lt(val1, val2))
                    },
                    super::BinOp::LShift => {
                        generator.add(Instruction::Lsh(val1, val2))
                    },
                    super::BinOp::RShift => {
                        generator.add(Instruction::Rsh(val1, val2))
                    }
                };
            }
        }
    }
}

impl Generate for PrimaryExpr {
    fn generate(&self, generator : &mut IrGenerator) {
        match self {
            PrimaryExpr::Call(c) => c.generate(generator),
            PrimaryExpr::Ident(name) => {
                let id = generator.get_varid(name.to_string()).expect(format!("unknown variable : {}", name).as_str());
                generator.add(Instruction::Varget(id));
            },
            PrimaryExpr::Litteral(val) => {
                generator.add(Instruction::Iconst(*val));
                ()
            },
            PrimaryExpr::Expr(e) => e.generate(generator)
        }
    }
}

impl Generate for Call {
    fn generate(&self, generator : &mut IrGenerator) {
        for arg in &self.1 {
            arg.generate(generator);
            let val = generator.get_current_block().unwrap().last_index();
            generator.add(Instruction::Pusharg(val));
        }
        if let Some(id) = generator.get_externs().iter().enumerate().filter(|x| x.1.0 == self.0).next().map(|x| x.0) {
            generator.add(Instruction::Call(id as u64));
        }else {
            let sig = JOLANG_STD.iter()
                .filter(|x| x.0 == self.0)
                .next().map(|x| &x.1)
                .expect(format!("unknown function : {}", self.0).as_str());
            let id = generator.decl_extern(self.0.clone(), sig);
            generator.add(Instruction::Call(id));
        }
    }
}
