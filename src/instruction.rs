// This file is part of "x64asm"
// Under the MIT License
// Copyright (c) 2022 Antonin Hérault

use crate::mnemonic::Mnemonic;
use crate::operand::Operand;

/// An Assembly code line but not formatted \
/// Pretty way to make Assembly code without a string
///
/// NOTE A comment could be put at the end of the line
pub struct Instruction {
    pub mnemonic: Mnemonic,
    pub operand1: Operand,
    pub operand2: Operand,
    pub comment: Option<String>,
}

impl Instruction {
    /// Constructor for instruction without operands
    pub fn new(mnemonic: Mnemonic) -> Self {
        Self {
            mnemonic,
            operand1: Operand::None,
            operand2: Operand::None,
            comment: None,
        }
    }
    
    /// Constructor for instruction with only 1 operand
    pub fn new_1(mnemonic: Mnemonic, operand1: Operand) -> Self {
        Self {
            mnemonic,
            operand1,
            operand2: Operand::None,
            comment: None,
        }
    }

    /// Constructor for instruction with 2 operands
    pub fn new_2(mnemonic: Mnemonic, operand1: Operand, operand2: Operand) -> Self {
        Self {
            mnemonic,
            operand1,
            operand2,
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

        // In case it's an instruction with no operands
        if self.operand1 == Operand::None && self.operand2 == Operand::None {
            formatted += &format!("{}", self.mnemonic.to_string())
        }

        // In case it's an instruction with only one operand
        else if self.operand2 == Operand::None {
            formatted += &format!(
                "{} {}", 
                self.mnemonic.to_string(),
                self.operand1.to_string(),
            )
        }

        // In case it's an instruction with two operands
        else {
            formatted += &format!(
                "{} {}, {}", 
                self.mnemonic.to_string(),
                self.operand1.to_string(),
                self.operand2.to_string(),
            )
        }

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
