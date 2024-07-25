use std::vec::Vec;
use super::{instructions::Instructions, source_file::SourceFile};

#[derive(Debug,Clone)]
pub struct TextData {
    instructions : Vec<Instructions>,
    /// jump table : (index in jump table), (instruction index) 
    jumps : Vec<(usize,usize)>
}

impl TryFrom<&SourceFile> for TextData {
    type Error = String;
    fn try_from(value: &SourceFile) -> Result<Self, Self::Error> {
        if value.text_start == None {
            return Err("you need to parse headers first".to_string())
        }
        let symbols : Vec<char> = value.lines()
            .enumerate()
            .skip(value.text_start.unwrap().try_into().unwrap())
            .skip(1)
            .filter_map(|(line,x)| if (value.text_end == None) || (line<(value.text_end.unwrap() as usize)) { Some(x) } else { None })
            .filter_map(|x| x.ok().map(|y| String::from(y)))
            .filter(|x| !x.starts_with("#"))
            .flat_map(|x| x.chars().collect::<Vec<_>>())
            .filter(|x: &char| !x.is_whitespace())
            .collect();
        let mut instructions = Vec::new();
        let mut jump_index : usize = 0;
        let mut last_label : usize = 0;
        let mut jumps : Vec<(usize,usize)>= vec![(0,0)]; 
        for symbol in symbols {
            jump_index += 1;
            instructions.push(match symbol {
                '<' => Instructions::Backward,
                '>' => Instructions::Forward,
                'L' => Instructions::Load,
                'S' => Instructions::Store,
                '+' => Instructions::Add,
                '-' => Instructions::Sub,
                '*' => Instructions::Mul,
                '/' => Instructions::Div,
                'P' => Instructions::Print,
                ']' => Instructions::Jump,
                '}' => Instructions::JumpIfZero,
                'E' => Instructions::Exit,
                'I' => Instructions::Inc,
                'D' => Instructions::Dec,
                'C' => Instructions::Compare,
                '[' => {
                    jump_index -= 1;
                    jumps.push((last_label,jump_index));
                    last_label += 1;
                    continue;
                }
                _ => {
                    return Err(format!("bad instruction : {}", symbol.to_string()).to_string()) 
                }
            })
        }
        Ok(TextData {
            instructions,
            jumps
        })
    }
}
