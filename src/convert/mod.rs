// This file is part of "x64asm"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

mod separator;

pub use separator::Separator;

use crate::{Instruction, instruction::Mnemonic};

/// Trait to convert anything into assembly code or part of code. 
pub trait ToAssembly {
    /// Converts anything into a `String`, which is necessarily a valid 
    /// assembly or a part of code. 
    fn to_assembly(&self, separator: Separator) -> String;
}

/// Converts all the instructions into assembly syntax-valid code calling the 
/// same function.
impl ToAssembly for Vec<Instruction> {
    fn to_assembly(&self, separator: Separator) -> String {
        self.into_iter().map(|instruction| {
            let start = match &instruction.mnemonic {
                Mnemonic::Label(_) => "",
                Mnemonic::Section(_) => "",
                _ => "\t",
            };

            format!("{}{}\n", start, instruction.to_assembly(separator))
        })
        .collect::<String>()
    }
}
