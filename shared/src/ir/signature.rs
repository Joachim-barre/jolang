use std::collections::LinkedList;

#[derive(Debug, PartialEq, Eq)]
pub struct Signature {
    ret : String,
    args : LinkedList<String>
}

impl Signature {
    pub fn new(ret : String, args : LinkedList<String>) -> Self {
        Self {
            ret,
            args
        }
    }

    pub fn to_string(&self) -> String{
        self.args.iter()
            .fold(self.ret.clone(), |s1, s2| s1 + "/" + s2)
    }
}
