use super::instructions::Instructions;

#[derive(Debug)]
pub struct Object {
    pub main_jump : u64,
    pub jumps : Vec<u64>,
    pub data : Vec<i64>,
    pub text : Vec<Instructions>
}
