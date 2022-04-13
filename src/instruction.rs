// This file is part of "x64asm"
// Under the MIT License
// Copyright (c) 2022 Antonin Hérault

use crate::mnemonic::Mnemonic;
use crate::operand::{ Operand, operand_vec_to_string };

/// An Assembly code line but not formatted \
/// Pretty way to make Assembly code without a string
///
/// NOTE A comment could be put at the end of the line
pub struct Instruction {
    pub mnemonic: Mnemonic,
    pub operands: Vec<Operand>,
    pub comment: Option<String>,
}

#[macro_export]
macro_rules! instruction {
    ($mnemonic:expr) => {
        x64asm::instruction::Instruction::new($mnemonic)
    };
    ($mnemonic:expr $(, $operands:expr)*) => {
        x64asm::instruction::Instruction {
            mnemonic: $mnemonic,
            operands: vec![ $($operands),* ],
            comment: None,
        }
    }
}

impl Instruction {
    /// Constructor for instruction without operands
    pub fn new(mnemonic: Mnemonic) -> Self {
        Self {
            mnemonic,
            operands: vec![],
            comment: None,
        }
    }

    /// Add a comment after the instruction \
    /// Function designed to be followed by another method
    pub fn with_comment(&mut self, comment: String) -> &mut Self {
        self.comment = Some(comment);
        self
    }

    /// NOTE Trait `std::string::ToString` was not used because it requires
    /// to have only the `self` argument
    pub fn to_string(&self, with_tabs: bool) -> String {
        let mut formatted = String::new();

        // In case it's an instruction with two operands
        formatted += &format!(
            "{} {}", 
            self.mnemonic.to_string(),
            operand_vec_to_string(&self.operands),
        );

        // Replace all spaces by a tabulation
        if with_tabs {
            formatted = formatted.replace(" ", "\t");
        }

        // Add a comment if specified 
        match &self.comment {
            Some(comment_string) => formatted += &format!(" ; {}", comment_string),
            None => {},
        }

        formatted
    }
}
