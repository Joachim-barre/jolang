use std::vec::Vec;
use super::{instructions::Instructions, source_file::SourceFile};

pub struct TextData {
    instructions : Vec<Instructions>,
    /// jump table : (index in jump table), (instruction index) 
    jumps : Vec<(usize,usize)>
}

