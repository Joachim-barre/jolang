use jolang_shared::{ffi::JolangExtern, ir::{instructions::{operand::{BlkId, FnId}, Instruction}, block::Block, IrObject}};
use core::panic;
use std::cell::{Ref, RefMut};
use index_list::{IndexList, ListIndex};
use crate::scope::Scope;

pub struct IrGenerator {
    object : IrObject,
    current_block : Option<BlkId>,
    current_pos : Option<ListIndex>,
    // data for the generation
    current_scopes : IndexList<Scope>
}

impl IrGenerator {
    pub fn new() -> Self {
        Self {
            object : IrObject::new(),
            current_block : None,
            current_pos : None,
            current_scopes : IndexList::new()
        }
    }

    pub fn decl_var(&mut self, name : String) -> u64 {
        let offset = self.get_current_block().map_or(0, |b| b.stack_size - 1);
        self.current_scopes.get_mut_first().map(|x| x.decl_var(name, offset));
        offset
    }

    pub fn update_var(&mut self, name : String) -> u64 {
        let offset = self.get_current_block().unwrap().stack_size - 1;
        let mut index = self.current_scopes.first_index();
        while index.is_some(){
            let scope = self.current_scopes.get_mut(index).unwrap();
            if scope.get_var(&name).is_some() {
                scope.update_var(&name, offset);
                return offset;
            }
            index = self.current_scopes.next_index(index);
        }
        panic!("unknown variable : {}", name);
    }

    pub fn into_ir(self) -> IrObject{
        self.object
    }

    pub fn get_current_block<'b>(&'b self) -> Option<Ref<'b, Block>> {
        self.current_block.as_ref().map(|id| self.object.get_block(*id))
    }

    pub fn get_current_block_mut<'b>(&'b self) -> Option<RefMut<'b, Block>> {
        self.current_block.as_ref().map(|id| self.object.get_block_mut(*id))
    }

    pub fn add(&mut self, i : Instruction) -> Option<ListIndex> {
        match i {
            Instruction::Iconst(_)
                | Instruction::Dup()
                | Instruction::Dupx(_)
                => self.inc_stack(),
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
                | Instruction::Swap()
                => None,
            Instruction::Call(f) => {
                if let Some(argc) = self.object.ext_fn.get(f as usize).map(|x| x.1)  {
                    for _ in 0..argc {
                        self.dec_stack();
                    }
                }
                if self.object.ext_fn.get(f as usize).map_or(false, |x| x.2) {
                    self.inc_stack()
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
        self.object.append_block(self.var_count() as u8)
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
            .filter_map(|s| s.get_var(&name))
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
        &self.object.ext_fn
    }

    pub fn decl_extern(&mut self, name : String, func : &Box<dyn JolangExtern>) -> FnId{
        self.object.decl_extern(name, func)
    }

    pub fn var_count(&self) -> usize {
        self.current_scopes.iter()
            .map(|x| x.var_count())
            .reduce(|x1, x2| x1 + x2).unwrap_or(0)
    }

    pub fn stack_size(&self) -> Option<u64> {
        self.get_current_block().map(|b| b.stack_size)
    }

    pub fn inc_stack(&mut self) -> Option<u64>{
        self.get_current_block_mut()
            .map(|mut x| {x.stack_size = x.stack_size + 1; x.stack_size})
    }

    pub fn dec_stack(&mut self) -> Option<u64>{
        self.get_current_block_mut()
            .map(|mut x| {x.stack_size = x.stack_size - 1; x.stack_size})
    }


    pub fn pass_vars(&mut self) {
        // the var to pass are alredy passed so we do not need to duplicate them
        if self.get_current_block().map_or(false, |b| b.instructions.is_empty()) {
            return
        }
        let offsets : Vec<_> = self.current_scopes.iter()
            .flat_map(|s| s.get_vars().values())
            .map(|v| *v)
            .collect();
        for v in offsets{
            self.add(Instruction::Dupx(v as i64));
        }
    }

    pub fn recive_vars(&mut self) {
        let mut pos : u64 = 0;
        let mut index = self.current_scopes.first_index();
        while index.is_some() {
            self.current_scopes.get_mut(index).unwrap().get_vars_mut().values_mut()
                .for_each(|x| {
                    *x = pos;
                    pos = pos + 1;
                });
            index =  self.current_scopes.next_index(index);
        }
    } 
}

pub trait Generate {
    fn generate(&self, generator : &mut IrGenerator);
}
