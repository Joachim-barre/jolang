use jolang_shared::ffi::jolang_std::JOLANG_STD;
use jolang_shared::ir::instructions::Instruction;
use crate::scope::{Scope, ScopeKind};
use crate::generator::{self, Generate, IrGenerator};
use super::{Call, Expr, Ident, PrimaryExpr, Program, Statement, UnaryOp};

impl Generate for Program {
    fn generate(&self, generator : &mut IrGenerator, size_hint : Option<u64>) {
        let blk = generator.append_block();
        let exit_block = generator.append_block();
        generator.goto_begin(exit_block);
        generator.add(Instruction::Iconst(64, 0)).unwrap();
        generator.add(Instruction::Reti());
        let scope = Scope::new(ScopeKind::Root, blk, exit_block);
        generator.enter_scope(scope);
        generator.goto_begin(blk);
        for s in &self.0 {
            s.generate(generator, Some(64));
        }
        generator.exit_scope();
        match generator.get_current_block().map(|b| b.instructions.get_last().map_or(Instruction::Dup(), |i| *i)) {
            Some(i) => match i {
                Instruction::Br(_)
                    | Instruction::Ret()
                    | Instruction::Reti()
                    | Instruction::Briz(_, _)
                    => return,
                _ => ()
            }
            None => ()
        }
        generator.add(Instruction::Br(exit_block));
    }
}

impl Generate for Statement {
    // size hint is used as the expected return value size (a none as size hint is interpreted as (i64)
    fn generate(&self, generator : &mut IrGenerator, size_hint : Option<u64>) {
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
                    s.generate(generator, size_hint);
                }
                generator.exit_scope();
                generator.pass_vars();
                generator.add(Instruction::Br(after_block));
                generator.goto_begin(after_block);
                generator.recive_vars();
            },
            Self::If(expr, then, _else) => {
                let then_block = generator.append_block();
                let else_block = generator.append_block();
                let after_block = match _else {
                    Some(_) => generator.append_block(),
                    None => else_block
                };
                generator.pass_vars();
                expr.generate(generator, None);
                generator.add(Instruction::Briz(else_block, then_block));
                generator.goto_begin(then_block);
                generator.recive_vars();
                
                let scope = Scope::new(ScopeKind::Block, then_block, after_block);
                generator.enter_scope(scope);
                then.generate(generator, size_hint);
                generator.exit_scope();
                generator.pass_vars();
                generator.add(Instruction::Br(after_block));
                
                generator.goto_end(else_block);
                if let Some(_else) = _else {
                    generator.pass_vars();
                    let scope = Scope::new(ScopeKind::Block, else_block, after_block);
                    generator.enter_scope(scope);
                    _else.generate(generator, size_hint);
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
                generator.pass_vars();
                expr.generate(generator, None);
                generator.add(Instruction::Briz(after_block, while_body));
                generator.goto_begin(while_body);
                generator.recive_vars();
                body.generate(generator, size_hint);
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
                body.generate(generator, size_hint);
                generator.exit_scope();
                generator.pass_vars();
                generator.add(Instruction::Br(loop_body));
                generator.goto_begin(after_block);
                generator.recive_vars();
            },
            Self::Return(expr) => {
                expr.generate(generator, size_hint);
                let target_size = match size_hint {
                    Some(s) => s,
                    None => 64
                };
                if generator.get_current_block()
                    .and_then(|b| b.stack_types.last().copied())
                    .unwrap_or(64) != target_size {
                    generator.add(Instruction::Icast(target_size));
                }
                generator.add(Instruction::Reti());
            },
            Self::Continue => {
                let mut to_exit = Vec::new();
                let mut found = false;
                for s in generator.get_scopes_mut().drain_iter() {
                    to_exit.push(s);
                    if to_exit[to_exit.len()-1].kind == ScopeKind::Loop {
                        found = true;
                        break;
                    }
                }
                if !found {
                    panic!("can't continue outside a loop");
                }
                generator.pass_vars();
                generator.add(Instruction::Br(to_exit[to_exit.len()-1].block));
                for s in to_exit.into_iter().rev() {
                    generator.enter_scope(s);
                }
            },
            Self::Break => {
               let mut to_exit = Vec::new();
                let mut found = false;
                for s in generator.get_scopes_mut().drain_iter() {
                    to_exit.push(s);
                    if to_exit[to_exit.len()-1].kind == ScopeKind::Loop {
                        found = true;
                        break;
                    }
                }
                if !found {
                    panic!("can't break outside a loop");
                }
                generator.pass_vars();
                generator.add(Instruction::Br(to_exit[to_exit.len()-1].exit));
                for s in to_exit.into_iter().rev() {
                    generator.enter_scope(s);
                }
            },
            Self::VarDecl(_type, name, value) => {
                let size = match _type {
                    Some(t) => Some(match t.as_str() {
                        "i8" => 8,
                        "i16" => 16,
                        "i32" => 32,
                        "i64" => 64,
                        "i128" => 128,
                        _ => panic!("unsupported type {}", t)
                    }),
                    None => None
                };
                let size = match value {
                    Some(v) => {
                        v.generate(generator, Some(32));
                        match size {
                            Some(s) => {
                                generator.add(Instruction::Icast(s));
                                s
                            },
                            None => generator
                                .get_current_block()
                                .and_then(|b| b.stack_types.last().copied())
                                .unwrap_or(32),
                        }
                    },
                    None => { 
                        let size = size.unwrap_or(32);
                        generator.add(Instruction::Iconst(size, 0)); 
                        size
                    }
                };
                generator.decl_var(name.to_string(), size);
            },
            Self::VarSet(name, value) => {
                let target_size = generator.get_var_size(name.to_string());
                value.generate(generator, target_size);
                if let Some(size) = generator.get_current_block()
                    .and_then(|b| b.stack_types.last().copied())
                    .and_then(|s| generator.get_var_size(name.to_string()).map(|s2| (s, s2)))
                    .and_then(|(s1,s2)| if s1!=s2 { Some(s2) } else { None })  {
                    generator.add(Instruction::Icast(size));
                }
                generator.update_var(name.to_string());
            },
            Self::Call(call) => {
                call.generate(generator, None);
            }
        }
    }
}

impl Generate for Expr {
    fn generate(&self, generator : &mut IrGenerator, size_hint : Option<u64>) {
        match self {
            Expr::PrimaryExpr(p) => p.generate(generator, size_hint),
            Expr::UnaryExpr(op, p) => {
                p.generate(generator, size_hint);
                match op {
                    UnaryOp::Plus => (),
                    UnaryOp::Minus => {
                        generator.add(Instruction::Neg());
                        ()
                    }
                };
            },
            Expr::BinExpr(e1, e2, op) => {
                e1.generate(generator, size_hint);
                e2.generate(generator, size_hint);
                let sizes = generator.get_current_block()
                    .and_then(|b| b.stack_types.last_chunk::<2>().cloned())
                    .unwrap_or([32;2]);
                let max_size = sizes.iter().max().copied().unwrap_or(32);
                if sizes[0] != max_size {
                    generator.add(Instruction::Swap());
                    generator.add(Instruction::Icast(max_size));
                    generator.add(Instruction::Swap());
                }else if sizes[1] != max_size {
                    generator.add(Instruction::Icast(max_size));
                }
                match op {
                    super::BinOp::Add => {
                        generator.add(Instruction::Add())
                    },
                    super::BinOp::Sub => {
                        generator.add(Instruction::Sub())
                    },
                    super::BinOp::Mul => {
                        generator.add(Instruction::Mul())
                    },
                    super::BinOp::Div => {
                        generator.add(Instruction::Div())
                    },
                    super::BinOp::Equal => {
                        generator.add(Instruction::Eq())
                    },
                    super::BinOp::NotEqual => {
                        generator.add(Instruction::Ne())
                    },
                    super::BinOp::Greater => {
                        generator.add(Instruction::Gt())
                    },
                    super::BinOp::GreaterEqual => {
                        generator.add(Instruction::Ge())
                    },
                    super::BinOp::LesserEqual => {
                        generator.add(Instruction::Le())
                    },
                    super::BinOp::Lesser => {
                        generator.add(Instruction::Lt())
                    },
                    super::BinOp::LShift => {
                        generator.add(Instruction::Lsh())
                    },
                    super::BinOp::RShift => {
                        generator.add(Instruction::Rsh())
                    }
                };
            }
        }
    }
}

impl Generate for PrimaryExpr {
    fn generate(&self, generator : &mut IrGenerator, size_hint : Option<u64>) {
        match self {
            PrimaryExpr::Call(c) => c.generate(generator, size_hint),
            PrimaryExpr::Ident(name) => {
                let offset = generator.get_var_offset(name.to_string()).expect(format!("unknown variable : {}", name).as_str());
                generator.add(Instruction::Dupx(offset));
            },
            PrimaryExpr::Litteral(val) => {
                let target_size = match size_hint {
                    Some(s) => s,
                    None => 128
                };
                generator.add(Instruction::Iconst(target_size, *val as u128));
                ()
            },
            PrimaryExpr::Expr(e) => e.generate(generator, size_hint)
        }
    }
}

impl Generate for Call {
    fn generate(&self, generator : &mut IrGenerator, size_hint : Option<u64>) {
        if let Some(id) = generator.get_externs().iter().enumerate().filter(|x| x.1.0 == self.0).next().map(|x| x.0) {
            for arg in &self.1 {
                arg.generate(generator, Some(64));
                if generator.get_current_block()
                    .and_then(|b| b.stack_types.last().copied())
                    .map_or(false, |x| x != 64)
                {
                    generator.add(Instruction::Icast(64));
                }
            }
            generator.add(Instruction::Call(id as u64));
        }else {
            let sig = JOLANG_STD.iter()
                .filter(|x| x.0 == self.0)
                .next().map(|x| &x.1)
                .expect(format!("unknown function : {}", self.0).as_str());
            let id = generator.decl_extern(self.0.clone(), sig);
            for arg in &self.1 {
                arg.generate(generator, Some(64));
            }
            generator.add(Instruction::Call(id));
        }
    }
}
