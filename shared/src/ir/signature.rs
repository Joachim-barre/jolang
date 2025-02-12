#[derive(Debug, PartialEq, Eq)]
pub struct Signature {
    pub ret : String,
    pub args : Vec<String>
}

impl Signature {
    pub fn new(ret : String, args : Vec<String>) -> Self {
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
