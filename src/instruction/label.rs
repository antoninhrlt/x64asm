// This file is part of "x64asm"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use crate::convert::{ ToAssembly, Separator };

/// Creates a `Mnemonic::Label`.
#[macro_export]
macro_rules! label {
    ($label:expr) => {
        instruction::Mnemonic::Label(instruction::Label::with_colon($label))
    }
}

/// Creates an `Operand::Label` from a `&str` label.
#[macro_export]
macro_rules! operand_label {
    ($label:expr) => {
        instruction::Operand::Label($label.to_string())
    }
}

/// Label for a function, with or without a colon.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Label {
    label: String,
    colon: bool,
}

impl Label {
    pub fn new(label: &str, colon: bool) -> Self {
        Self {
            label: label.to_string(),
            colon,
        }
    }

    pub fn with_colon(label: &str) -> Self {
        Self::new(label, true)
    }
    
    pub fn without_colon(label: &str) -> Self {
        Self::new(label, false)
    }
}

impl ToAssembly for Label {
    fn to_assembly(&self, separator: Separator) -> String {
        format!("{}{}", self.label, if self.colon {":"} else {""})
    }
}
