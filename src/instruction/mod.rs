// This file is part of "x64asm"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

mod ddirective;
mod label;
mod mnemonic;
mod operand;
mod register;
mod section;

pub use ddirective::DDirective;
pub use ddirective::DDirective as DD;
pub use label::Label;
pub use mnemonic::Mnemonic;
pub use operand::Operand;
pub use operand::Operand as Op;
pub use register::Register;
pub use register::Register as Reg;
pub use section::Section;

use crate::convert::{ Separator, ToAssembly };

/// Creates an `Instruction` thanks to `InstructionBuilder`.
/// ## Example
/// ```rust
/// use x64asm::instruction;
/// 
/// let syscall = instruction!(Mnemonic::Syscall);
/// let instruction = instruction!(Mnemonic::Mov, Register::Rax, Register::Rbx);
/// ```
#[macro_export]
macro_rules! _instruction {
    ($mnemonic:expr) => {
        InstructionBuilder::new($mnemonic).create()
    };
    ($mnemonic:expr $(, $operands:expr)*) => {
        InstructionBuilder::new($mnemonic)
            .with_operands(&[ $($operands),* ])
            .create()
    }
}

/// Creates an `Instruction` object step by step.
pub struct InstructionBuilder {
    /// The instruction to build, initially set with no operands and no vector 
    /// by the `Self::new()` function. 
    instruction: Instruction,
}

impl InstructionBuilder {
    /// Creates an `InstructionBuilder` object with an instruction with no 
    /// operands and no comment.
    pub fn new(mnemonic: Mnemonic) -> Self {
        Self {
            instruction: Instruction {
                mnemonic,
                operands: vec![],
                comment: None,
            }
        }
    }

    /// Follows the instruction's mnemonic with operands.
    /// ## Example
    /// ```rust
    /// let instruction = InstructionBuilder::new(Mov)
    ///     .with_operands(&[Register::Rax, Register::Rbx])
    ///     .create();
    /// ```
    pub fn with_operands(&mut self, operands: &[Operand]) -> &mut Self {
        self.instruction.operands = operands.to_vec();
        self
    }

    /// Follows the instruction with a comment.
    /// ## Example
    /// ```rust
    /// let instruction = InstructionBuilder::new(Mov)
    ///     .with_comment("Mov instruction")
    ///     .create();
    /// ```
    pub fn with_comment(&mut self, comment: &str) -> &mut Self {
        self.instruction.comment = Some(comment.to_string());
        self
    }

    /// Finishes building the instruction.
    /// 
    /// Returns an `Instruction` object, which is the private field 
    /// `self.instruction` of this structure.
    pub fn create(&mut self) -> Instruction {
        self.instruction.clone()
    }
}

/// Instruction to be transformed into actual assembly code.
#[derive(Clone)]
pub struct Instruction {
    /// The instruction's mnemonic. It's where the instruction starts.
    pub mnemonic: Mnemonic,
    /// The operands following the mnemonic for the instruction.
    /// The vector can be null, some of mnemonics don't require operands.
    pub operands: Vec<Operand>,
    /// The comment at the end of the line. When `None`, no comment is added. 
    pub comment: Option<String>,
}

impl ToAssembly for Instruction {
    fn to_assembly(&self, separator: Separator) -> String {
        let mut assembly = String::new();

        assembly += &format!(
            "{}{}{}", 
            self.mnemonic.to_assembly(separator),
            separator.value(),
            self.operands.to_assembly(separator),
        );

        // Add a comment at the end if specified.
        match &self.comment {
            Some(comment) => assembly += &format!(" ; {}", comment),
            None => {},
        }

        assembly
    }
}
