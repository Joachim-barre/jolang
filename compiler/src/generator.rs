use jolang_shared::{ffi::JolangExtern, ir::{instructions::{operand::{BlkId, FnId, VarId}, Instruction}, Block, IrObject}};
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

    pub fn decl_var(&mut self, name : String, value : i64) -> VarId {
        let id = self.object.decl_var(value);
        self.current_scopes.get_mut_first().map(|x| x.decl_var(name, id));
        id
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
        let pos = self.get_current_block_mut().map(|mut b| match self.current_pos{
            Some(pos) => {
                b.insert_after(pos, i)
            },
            None => {
                b.insert_first(i)
            }
        });
        self.current_pos = pos;
        pos
    }

    pub fn append_block(&mut self) -> BlkId {
        self.object.append_block()
    }
    
    pub fn goto_end(&mut self, block : BlkId) {
        self.current_block = Some(block);
        let pos = self.get_current_block().map(|x| x.last_index());
        self.current_pos = pos;
    }

    pub fn goto_begin(&mut self, block : BlkId) {
        self.current_block = Some(block);
        let pos = self.get_current_block().map(|x| x.first_index());
        self.current_pos = pos;
    }

    pub fn enter_scope(&mut self, scope : Scope) {
        self.current_scopes.insert_first(scope);
    }

    pub fn get_varid(&mut self, name : String) -> Option<VarId> {
        self.current_scopes.iter()
            .map(|s| s.get_var(&name))
            .next()?
    }

    pub fn exit_scope(&mut self) {
        self.current_scopes.remove_first();
    }

    pub fn get_scopes(&self) -> &IndexList<Scope> {
        &self.current_scopes
    }

    pub fn get_externs(&self) -> &Vec<(String, u8, bool)> {
        &self.object.ext_fn
    }

    pub fn decl_extern(&mut self, name : String, func : &Box<dyn JolangExtern>) -> FnId{
        self.object.decl_extern(name, func)
    }
}

pub trait Generate {
    fn generate(&self, generator : &mut IrGenerator);
}
