// This file is part of "x64asm"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

pub mod convert;
pub mod instruction;

pub use instruction::{ Instruction, InstructionBuilder };

// Macros
pub use ddirective as dd;
pub use operand_label as oplabel;
pub use _instruction as instruction;
pub use _instruction as i;
pub use operand_string as opstring;
pub use operand_expression as opexpr;
pub use register as reg;

/// Permit to use the macros easily.
/// ## Example
/// ```rust
/// use x64asm::macros::*;
/// 
/// i!(Mov, reg!(Rax), Op::Literal(1));
/// ```
/// and not :
/// ```
/// use x64asm::{ i, reg };
/// use x64asm::{ instruction, InstructionBuilder };
/// use x64asm::instruction::{ Mnemonic, Register, Op };
///
/// i!(Mnemonic::Mov, reg!(Register::Rax), Op::Literal(1));
/// ```
pub mod macros {
    pub use super::{ instruction, InstructionBuilder };
    pub use super::instruction::{ Mnemonic::*, Section::*, Reg::*, DD::*, Op };
    pub use crate::{ dd, oplabel, i, opstring, opexpr, reg, section, label };
}

#[cfg(test)]
mod tests {
    #[test]
    fn example() {
        use std::path::Path;
        use std::fs::File;
        use std::io::prelude::*;

        use super::convert::{ ToAssembly, Separator };
        use super::macros::*;

        let instructions = vec![
            i!(Global, oplabel!("_start")),

            i!(section!(Text)),
            i!(label!("_start")),
            i!(Mov, reg!(Rax), Op::Literal(1)),
            i!(Mov, reg!(Rdi), Op::Literal(1)),
            i!(Mov, reg!(Rsi), oplabel!("msg")),
            i!(Mov, reg!(Rdx), oplabel!("msg_len")),
            i!(Syscall),

            i!(Mov, reg!(Rax), Op::Literal(60)),
            i!(Mov, reg!(Rdi), Op::Literal(0)),
            i!(Syscall),
            
            i!(section!(Data)),
            i!(label!("msg"), dd!(Db), opstring!("Hello world")),
            i!(label!("msg_len"), dd!(Equ), opexpr!("$ - msg")),
        ];

        let code = instructions.to_assembly(Separator::Space);
        let mut stream = File::create(&Path::new("target/test.asm")).unwrap();
        write!(stream, "{}", code).unwrap();
    }

    // /// This won't work with the new version of "x64asm". Here is a vestige of what
    // /// it looked like before :
    // #[test]
    // fn old_example() {
    //     use super::{
    //         ddirective, ddirective::DefineDirective::*, 
    //         formatter::Formatter, 
    //         instruction as i, label,
    //         mnemonic::Mnemonic::*, 
    //         operand::Op, 
    //         reg, register::Register::*, 
    //         section, section::Section::*,
    //     };

    //     let mut x64asm_formatter = Formatter::new(false); // false : do not replace spaces by tabulations
    
    //     x64asm_formatter.add_instructions(&mut vec![
    //         i!(Global, Op::Label("_start".to_string())),
        
    //         i!(section!(Text)),
        
    //         i!(label!("_start")),
    //         i!(Mov, reg!(Rax), Op::Literal(1)),
    //         i!(Mov, reg!(Rdi), Op::Literal(1)),
    //         i!(Mov, reg!(Rsi), Op::Label("msg".to_string())),
    //         i!(Mov, reg!(Rdx), Op::Label("msg_len".to_string())),
    //         i!(Syscall),
        
    //         i!(Mov, reg!(Rax), Op::Literal(60)),
    //         i!(Mov, reg!(Rdi), Op::Literal(0)),
    //         i!(Syscall),
        
    //         i!(section!(Data)),
        
    //         i!(label!("msg"), ddirective!(Db), Op::String("Hello world".to_string())),
    //         i!(label!("msg_len"), ddirective!(Equ), Op::Expression("$ - msg".to_string())),
    //     ]);
    
    //     x64asm_formatter.to_file(&Path::new("target/test.asm")).unwrap();
    // }
}
