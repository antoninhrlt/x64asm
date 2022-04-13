// This file is part of "x64asm"
// Under the MIT License
// Copyright (c) 2022 Antonin Hérault

use std::path::Path;

use x64asm::{
    ddirective, ddirective::DefineDirective::*, 
    formatter::Formatter, 
    instruction as i, label,
    mnemonic::Mnemonic::*, 
    operand::Op, 
    reg, register::Register::*, 
    section, section::Section::*,
};

fn main() {
    let mut x64asm_formatter = Formatter::new(false); // false : do not replace spaces by tabulations

    x64asm_formatter.add_instructions(&mut vec![
        i!(Global, Op::Label("_start".to_string())),
    
        i!(section!(Text)),
    
        i!(label!("_start")),
        i!(Mov, reg!(Rax), Op::Literal(1)),
        i!(Mov, reg!(Rdi), Op::Literal(1)),
        i!(Mov, reg!(Rsi), Op::Label("msg".to_string())),
        i!(Mov, reg!(Rdx), Op::Label("msg_len".to_string())),
        i!(Syscall),
    
        i!(Mov, reg!(Rax), Op::Literal(60)),
        i!(Mov, reg!(Rdi), Op::Literal(0)),
        i!(Syscall),
    
        i!(section!(Data)),
    
        i!(label!("msg"), ddirective!(Db), Op::String("Hello world".to_string())),
        i!(label!("msg_len"), ddirective!(Equ), Op::Expression("$ - msg".to_string())),
    ]);

    x64asm_formatter.to_file(&Path::new("test.asm")).unwrap();
}
