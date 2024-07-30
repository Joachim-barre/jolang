use crate::{cli::run::RunArgs, commons::{instructions::Instructions, object::Object}};
use std::{result::Result, fs::{File, OpenOptions}};
use inkwell::{context::Context, AddressSpace, OptimizationLevel, execution_engine::JitFunction};
pub mod externs;

pub fn run(args : RunArgs) -> Result<(), String> {
    if !args.file.is_local() {
        return Err(String::from("please input a local file"))
    }
    let mut file : File;
    match OpenOptions::new().read(true).write(false).truncate(false).append(false).open(args.file.clone().as_os_str()) {
        Ok(f) => {
            file = f
        }
        Err(_) => {
            return Err(String::from("can't open file"))
        }
    }
    println!("loading object {}", args.file);
    let object = Object::load(&mut file)?;
    
    let mut blocks : Vec<Vec<Instructions>> = Vec::new();

    let mut block_offset : usize = 0; 

    let mut current_block : Vec<Instructions> = object.text;

    for jump in &object.jumps[1..] {
        let next_block = current_block.split_off(*jump as usize - block_offset);
        block_offset += *jump as usize;
        blocks.push(current_block);
        current_block = next_block;
    }
    blocks.push(current_block);

    let data_size = object.data.len();

    let context = Context::create();
    let module = context.create_module("main");
    let builder = context.create_builder();
    let i64_type = context.i64_type();
    let ptr_type = i64_type.ptr_type(AddressSpace::default());
    let main_fn_type = i64_type.fn_type(&[i64_type.into(), ptr_type.into()], false);
    let main_fn = module.add_function("main", main_fn_type, None);
   
    let print_int_type = context.void_type().fn_type(&[i64_type.into()], false);
    let print_int_fn = module.add_function("print_int", print_int_type, None);

    let print_block = context.append_basic_block(print_int_fn, "link");
    builder.position_at_end(print_block);

    let print_int_ptr = i64_type
            .const_int(externs::print_int as *const usize as u64, false)
            .const_to_pointer(print_int_type.ptr_type(AddressSpace::default()));

    let _ = builder.build_indirect_call(print_int_type, print_int_ptr, &[print_int_fn.get_first_param().unwrap().into()], "call");
    let _ = builder.build_return(None); 

    let entry_block = context.append_basic_block(main_fn, "entry");
    let tape_size = main_fn.get_first_param().unwrap().into_int_value();
    let tape_start_ptr = main_fn.get_nth_param(1).unwrap().into_pointer_value();

    let blocks : Vec<_> = blocks.iter()
        .enumerate()
        .map(|(i, x)| (format!("block{}", i.to_string()), x))
        .map(|(i, x)| (context.append_basic_block(main_fn, &i), x))
        .collect();

    let switch_table : Vec<_> = blocks.iter()
        .map(|(block, _x)| block)
        .enumerate()
        .map(|(i, x)| (i64_type.const_int(i as u64, false), *x))
        .collect();

    builder.position_at_end(entry_block);
    let tape_ptr_ptr = builder.build_alloca(ptr_type, "tape_ptr_ptr").unwrap();
    let reg_ptr = builder.build_alloca(i64_type, "reg_ptr").unwrap();
    let _ = builder.build_store(tape_ptr_ptr, tape_start_ptr);
    let _ = builder.build_store(reg_ptr, i64_type.const_int(0, false));
    let one = i64_type.const_int(1, false);

    let exit_block = context.append_basic_block(main_fn, "exit");
    let mut block_terminated = false;

    for (block, instr) in blocks {
        if !block_terminated {
            let _ = builder.build_unconditional_branch(block);
        }
        block_terminated = false;
        builder.position_at_end(block);
        for i in instr {
            if block_terminated {
                return Err("unreachable code".to_string())
            }
            match *i{
                Instructions::Backward => {
                    let tape_ptr = builder.build_load(ptr_type, tape_ptr_ptr, "tape_ptr").unwrap().into_pointer_value();
                    let tape_ptr_int = builder.build_ptr_to_int(tape_ptr, i64_type, "tape_ptr_int").unwrap();
                    let tape_start_ptr_int = builder.build_ptr_to_int(tape_start_ptr, i64_type, "tape_start_ptr_int").unwrap();
                    let is_equal = builder.build_int_compare(inkwell::IntPredicate::EQ, tape_ptr_int, tape_start_ptr_int, "is_equal").unwrap();
                    let dec_tape_ptr_int = builder.build_int_sub(tape_ptr_int, one, "dec_tape_ptr_int").unwrap();
                    let max_tape_ptr_int = builder.build_int_add(tape_start_ptr_int, tape_size, "max_tape_ptr_int").unwrap();
                    let res_tape_ptr_int = builder.build_select(is_equal, max_tape_ptr_int, dec_tape_ptr_int, "res_tape_ptr_int").unwrap().into_int_value();
                    let res_tape_ptr = builder.build_int_to_ptr(res_tape_ptr_int, ptr_type, "res_tape_ptr").unwrap();
                    let _ = builder.build_store(tape_ptr_ptr, res_tape_ptr);
                },
                Instructions::Forward => {
                    let tape_ptr = builder.build_load(ptr_type, tape_ptr_ptr, "tape_ptr").unwrap().into_pointer_value();
                    let tape_ptr_int = builder.build_ptr_to_int(tape_ptr, i64_type, "tape_ptr_int").unwrap();
                    let tape_start_ptr_int = builder.build_ptr_to_int(tape_start_ptr, i64_type, "tape_start_ptr_int").unwrap();
                    let max_tape_ptr_int = builder.build_int_add(tape_start_ptr_int, tape_size, "max_tape_ptr_int").unwrap();
                    let is_equal = builder.build_int_compare(inkwell::IntPredicate::EQ, tape_ptr_int, max_tape_ptr_int, "is_equal").unwrap();
                    let inc_tape_ptr_int = builder.build_int_add(tape_ptr_int, one, "inc_tape_ptr_int").unwrap();
                    let res_tape_ptr_int = builder.build_select(is_equal, tape_start_ptr_int, inc_tape_ptr_int, "res_tape_ptr_int").unwrap().into_int_value();
                    let res_tape_ptr = builder.build_int_to_ptr(res_tape_ptr_int, ptr_type, "res_tape_ptr").unwrap();
                    let _ = builder.build_store(tape_ptr_ptr, res_tape_ptr);
                },
                Instructions::Load => {
                    let tape_ptr = builder.build_load(ptr_type, tape_ptr_ptr, "tape_ptr").unwrap().into_pointer_value();
                    let value = builder.build_load(i64_type, tape_ptr, "value").unwrap().into_int_value();
                    let _ = builder.build_store(reg_ptr, value);
                },
                Instructions::Store => {
                    let tape_ptr = builder.build_load(ptr_type, tape_ptr_ptr, "tape_ptr").unwrap().into_pointer_value();
                    let value = builder.build_load(i64_type, reg_ptr, "value").unwrap().into_int_value();
                    let _ = builder.build_store(tape_ptr, value);
                },
                Instructions::Add 
                | Instructions::Sub
                | Instructions::Mul
                | Instructions::Div => {
                    let tape_ptr = builder.build_load(ptr_type, tape_ptr_ptr, "tape_ptr").unwrap().into_pointer_value();
                    let reg_value = builder.build_load(i64_type, reg_ptr, "reg_value").unwrap().into_int_value(); 
                    let tape_value = builder.build_load(i64_type, tape_ptr, "tape_value").unwrap().into_int_value();
                    let res_value = match *i {
                        Instructions::Add => builder.build_int_add(reg_value, tape_value, "res_value").unwrap(),
                        Instructions::Sub => builder.build_int_sub(reg_value, tape_value, "res_value").unwrap(),
                        Instructions::Mul => builder.build_int_mul(reg_value, tape_value, "res_value").unwrap(),
                        Instructions::Div => builder.build_int_signed_div(reg_value, tape_value, "res_value").unwrap(),
                        _ => unreachable!() 
                    };
                    let _ = builder.build_store(reg_ptr, res_value);
                },
                Instructions::Print => {
                    let reg_value = builder.build_load(i64_type, reg_ptr, "reg_value").unwrap().into_int_value();
                    let _ = builder.build_call(print_int_fn, &[reg_value.into()], "_");
                },
                Instructions::Jump => {
                    let tape_ptr = builder.build_load(ptr_type, tape_ptr_ptr, "tape_ptr").unwrap().into_pointer_value();
                    let tape_value = builder.build_load(i64_type, tape_ptr, "tape_value").unwrap().into_int_value();
                    let _ = builder.build_switch(tape_value, switch_table[0].1, &switch_table[..]);
                    block_terminated = true;
                },
                Instructions::JumpIfZero => {
                    // split block
                    let block_name = String::from(builder.get_insert_block().unwrap().get_name().to_str().unwrap());
                    let switch_block = context.append_basic_block(main_fn, (block_name.clone() + "_switch").as_str());
                    let else_block = context.append_basic_block(main_fn, (block_name + "_then").as_str());
                    let _ = switch_block.move_after(builder.get_insert_block().unwrap());
                    let _ = else_block.move_after(switch_block);
                    let reg_value = builder.build_load(i64_type, reg_ptr, "reg_value").unwrap().into_int_value();
                    let is_zero = builder.build_int_compare(inkwell::IntPredicate::EQ, reg_value, i64_type.const_int(0, false), "is_equal").unwrap();
                    let _ = builder.build_conditional_branch(is_zero, switch_block, else_block);
                    builder.position_at_end(switch_block);
                    let tape_ptr = builder.build_load(ptr_type, tape_ptr_ptr, "tape_ptr").unwrap().into_pointer_value();
                    let tape_value = builder.build_load(i64_type, tape_ptr, "tape_value").unwrap().into_int_value();
                    let _ = builder.build_switch(tape_value, switch_table[0].1, &switch_table[..]);
                    builder.position_at_end(else_block);
                },
                Instructions::Exit => {
                    let _ = builder.build_unconditional_branch(exit_block);
                    block_terminated = true;
                },
                Instructions::Inc => {
                    let reg_value = builder.build_load(i64_type, reg_ptr, "reg_value").unwrap().into_int_value(); 
                    let res_value = builder.build_int_add(reg_value, one, "res_value").unwrap();
                    let _ = builder.build_store(reg_ptr, res_value); 
                },
                Instructions::Dec => {
                    let reg_value = builder.build_load(i64_type, reg_ptr, "reg_value").unwrap().into_int_value(); 
                    let res_value = builder.build_int_sub(reg_value, one, "res_value").unwrap();
                    let _ = builder.build_store(reg_ptr, res_value); 
                },
                Instructions::Compare => {
                    let tape_ptr = builder.build_load(ptr_type, tape_ptr_ptr, "tape_ptr").unwrap().into_pointer_value();
                    let reg_value = builder.build_load(i64_type, reg_ptr, "reg_value").unwrap().into_int_value(); 
                    let tape_value = builder.build_load(i64_type, tape_ptr, "tape_value").unwrap().into_int_value();
                    let eq = builder.build_int_compare(inkwell::IntPredicate::EQ, reg_value, tape_value, "eq").unwrap();
                    let gt = builder.build_int_compare(inkwell::IntPredicate::SGT, reg_value, tape_value, "gt").unwrap();
                    let res_value = builder.build_select(eq, i64_type.const_int(0, false), builder.build_select(gt, one, i64_type.const_int(u64::from_le_bytes((-1i64).to_le_bytes()), false), "res").unwrap().into_int_value(), "res_value").unwrap();
                    let _ = builder.build_store(reg_ptr, res_value);
                },
                _ => unreachable!()
            }
        }
    }
    
    if !block_terminated {
        let _ = builder.build_unconditional_branch(exit_block);
    }
    builder.position_at_end(exit_block);
    let reg_value = builder.build_load(i64_type, reg_ptr, "reg_value").unwrap().into_int_value();
    let _ = builder.build_return(Some(&reg_value));

    if args.print_to_stderr {
        module.print_to_stderr();
    }

    let execution_engine = module.create_jit_execution_engine(OptimizationLevel::Default).unwrap();

    unsafe {
        type MainFn = unsafe extern "C" fn(u64, *const i64) -> i64;
        let main_fn: JitFunction<MainFn> = execution_engine.get_function("main").unwrap();
        main_fn.call(data_size as u64, object.data.as_ptr());
    }

    Ok(())
}
