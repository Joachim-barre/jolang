mod block;
use jolang_shared::{ffi::JolangExtern, ir::{instructions::{operand::{BlkId, FnId}, Instruction}, IrObject}};
use core::panic;
use std::{borrow::{Borrow, BorrowMut}, cell::{RefCell, Ref, RefMut}};
use index_list::{IndexList, ListIndex};
use crate::scope::Scope;
use block::Block;

pub struct IrGenerator {
    blocks : Vec<RefCell<Block>>,
    ext_fn : Vec<(String, u8, bool)>,
    current_block : Option<BlkId>,
    current_pos : Option<ListIndex>,
    // data for the generation
    current_scopes : IndexList<Scope>
}

impl IrGenerator {
    pub fn new() -> Self {
        Self {
            blocks : Vec::new(),
            ext_fn : Vec::new(),
            current_block : None,
            current_pos : None,
            current_scopes : IndexList::new()
        }
    }

    pub fn decl_var(&mut self, name : String, size : u64) -> u64 {
        let offset = self.get_current_block().map_or(0, |b| b.stack_size() - 1);
        self.current_scopes.get_mut_first().map(|x| x.decl_var(name, offset, size));
        offset
    }

    pub fn update_var(&mut self, name : String) -> u64 {
        let offset = self.get_current_block().unwrap().stack_size() - 1;
        let mut index = self.current_scopes.first_index();
        while index.is_some(){
            let scope = self.current_scopes.get_mut(index).unwrap();
            if scope.get_var_offset(&name).is_some() {
                scope.update_var(&name, offset);
                return offset;
            }
            index = self.current_scopes.next_index(index);
        }
        panic!("unknown variable : {}", name);
    }

    pub fn into_ir(self) -> IrObject{
        IrObject { 
            blocks: self.blocks.iter()
                .map(|b| b.take().into_ir_block())
                .collect::<Vec<_>>(),
            ext_fn: self.ext_fn
        }
    }

    pub fn get_current_block<'b>(&'b self) -> Option<Ref<'b, Block>> {
        self.current_block.as_ref().and_then(|id| self.blocks.get(*id as usize).map(|b| b.borrow()))
    }

    pub fn get_current_block_mut<'b>(&'b self) -> Option<RefMut<'b, Block>> {
        self.current_block.as_ref().and_then(|id| self.blocks.get(*id as usize).map(|b| b.borrow_mut()))
    }

    pub fn add(&mut self, i : Instruction) -> Option<ListIndex> {
        match i {
            Instruction::Iconst(size, ..)
                => self.inc_stack(size),
            Instruction::Dup()
                => self.get_current_block()
                    .and_then(|b| b.stack_types.last().copied())
                    .and_then(|s| self.inc_stack(s)),
            Instruction::Icast(target)
                => self.get_current_block_mut()
                    .and_then(|mut b| b.stack_types.pop())
                    .and_then(|_| self.inc_stack(target)),
            Instruction::Dupx(pos)
                => self.get_current_block()
                    .and_then(|b| b.stack_types.get(pos as usize).copied())
                    .and_then(|s| self.inc_stack(s)),
            Instruction::Swap() 
                => self.get_current_block_mut()
                    .and_then(|mut b| b.stack_types.pop().and_then(|x| b.stack_types.pop().map(|x2| (x,x2))))
                    .and_then(|(s1,s2)| {self.inc_stack(s1); self.inc_stack(s2); None}),
            Instruction::Add()
                | Instruction::Sub()
                | Instruction::Mul()
                | Instruction::Div()
                | Instruction::Eq()
                | Instruction::Ne()
                | Instruction::Gt()
                | Instruction::Ge()
                | Instruction::Lt()
                | Instruction::Le()
                | Instruction::Lsh()
                | Instruction::Rsh()
                => self.dec_stack(),
            Instruction::Ret()
                | Instruction::Reti()
                | Instruction::Br(_)
                | Instruction::Neg()
                | Instruction::Briz(_, _)
                => None,
            Instruction::Call(f) => {
                if let Some(argc) = self.ext_fn.get(f as usize).map(|x| x.1)  {
                    for _ in 0..argc {
                        self.dec_stack();
                    }
                }
                if self.ext_fn.get(f as usize).map_or(false, |x| x.2) {
                    self.inc_stack(64)
                }else{
                    None
                }
            }
        };
        let pos = self.get_current_block_mut().map(|mut b| match self.current_pos{
            Some(pos) => {
                b.instructions.insert_after(pos, i)
            },
            None => {
                b.instructions.insert_first(i)
            }
        });
        match i {
            Instruction::Ret()
                | Instruction::Reti()
                | Instruction::Br(_)
                | Instruction::Briz(_, _)
            => {
                // exit the current block to prevent writting instrunctions in it
                self.current_block = None;
                self.current_pos = None;
                return pos
            },
            _ => ()
        }
        self.current_pos = pos;
        pos
    }

    pub fn append_block(&mut self) -> BlkId {
        self.blocks.push(RefCell::new(Block::new(self.var_sizes())));
            return (self.blocks.len() as BlkId) - 1
    }
    
    pub fn goto_end(&mut self, block : BlkId) {
        self.current_block = Some(block);
        let pos = self.get_current_block().map(|x| x.instructions.last_index());
        self.current_pos = pos;
    }

    pub fn goto_begin(&mut self, block : BlkId) {
        self.current_block = Some(block);
        let pos = self.get_current_block().map(|x| x.instructions.first_index());
        self.current_pos = pos;
    }

    pub fn enter_scope(&mut self, scope : Scope) {
        self.current_scopes.insert_first(scope);
    }

    // get a variable offset from the top of the stack
    pub fn get_var_offset(&mut self, name : String) -> Option<u64> {
        self.current_scopes.iter()
            .filter_map(|s| s.get_var_offset(&name))
            .next()
            .map(|x| x)
    }

    pub fn get_var_size(&mut self, name : String) -> Option<u64> {
        self.current_scopes.iter()
            .filter_map(|s| s.get_var_size(&name))
            .next()
            .map(|x| x)
    }



    pub fn exit_scope(&mut self) {
        self.current_scopes.remove_first();
    }

    pub fn get_scopes(&self) -> &IndexList<Scope> {
        &self.current_scopes
    }

    pub fn get_scopes_mut(&mut self) -> &mut IndexList<Scope> {
        &mut self.current_scopes
    }

    pub fn get_externs(&self) -> &Vec<(String, u8, bool)> {
        &self.ext_fn
    }

    pub fn decl_extern(&mut self, name : String, func : &Box<dyn JolangExtern>) -> FnId{
        self.ext_fn.push((name, func.arg_count(), func.returns()));
        (self.ext_fn.len() -1) as FnId
    }

    pub fn var_sizes(&self) -> Vec<u64> {
        self.current_scopes.iter()
            .flat_map(|x| x.var_sizes())
            .collect::<Vec<_>>()
    }

    pub fn stack_size(&self) -> Option<u64> {
        self.get_current_block().map(|b| b.stack_size())
    }

    pub fn inc_stack(&mut self, size : u64) -> Option<u64>{
        self.get_current_block_mut()
            .map(|mut x| {x.stack_types.push(size); x.stack_size()})
    }

    pub fn dec_stack(&mut self) -> Option<u64>{
        self.get_current_block_mut()
            .map(|mut x| {x.stack_types.pop(); x.stack_size()})
    }


    pub fn pass_vars(&mut self) {
        // the var to pass are alredy passed so we do not need to duplicate them
        if self.get_current_block().map_or(false, |b| b.instructions.is_empty()) {
            return
        }
        let offsets : Vec<_> = self.current_scopes.iter()
            .flat_map(|s| s.get_vars().values())
            .map(|v| v.0)
            .collect();
        for v in offsets{
            self.add(Instruction::Dupx(v));
        }
    }

    pub fn recive_vars(&mut self) {
        let mut pos : u64 = 0;
        let mut index = self.current_scopes.first_index();
        while index.is_some() {
            self.current_scopes.get_mut(index).unwrap().get_vars_mut().values_mut()
                .for_each(|x| {
                    x.0 = pos;
                    pos = pos + 1;
                });
            index =  self.current_scopes.next_index(index);
        }
    } 
}

pub trait Generate {
    fn generate(&self, generator : &mut IrGenerator);
}
