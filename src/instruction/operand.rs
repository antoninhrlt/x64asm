// This file is part of "x64asm"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use crate::convert::{ ToAssembly, Separator };
use super::Register;
use super::DDirective;

/// Creates an `Operand::String` from a `&str` string.
#[macro_export]
macro_rules! operand_string {
    ($string:expr) => {
        instruction::Operand::String($string.to_string())
    };
}

/// Creates an `Operand::Expression` from a `&str` expression.
#[macro_export]
macro_rules! operand_expression {
    ($expression:expr) => {
        instruction::Operand::Expression($expression.to_string())
    };
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operand {
    /// Think to use `use x64asm::register::Register::*;`
    Register(Register),
    Indirect(Register),
    Literal(i64),
    DDirective(DDirective),
    Label(String),
    String(String),

    Expression(String),

    Byte,
    Word,
    Dword,
    Qword,

    None, // to avoid the `Option` usage
}

impl ToAssembly for Vec<Operand> {
    fn to_assembly(&self, separator: Separator) -> String {
        let mut assembly = String::new();
        let mut is_variable = false;
    
        for (i, operand) in self.iter().enumerate() {
            assembly += &format!("{}", operand.to_assembly(separator));
    
            if is_variable && i != self.len() - 1 {
                assembly += ",";
            }
            if i == 0 {
                match operand {
                    Operand::DDirective(_) => is_variable = true,
                    _ => if self.len() != 1 { assembly += "," }, 
                }
            }
            if operand.to_assembly(separator) != String::new() { 
                assembly += " ";
            }
        }

        assembly
    }
}

impl ToAssembly for Operand {
    fn to_assembly(&self, _: Separator) -> String {
        match self {
            Operand::Register(reg) => reg.to_string(),
            Operand::Indirect(reg) => format!("[{}]", reg.to_string()),
            Operand::Literal(value) => value.to_string(),
            Operand::DDirective(dd) => dd.to_string(),
            Operand::Label(label) => label.to_string(),
            Operand::String(string) => format!("`{}`", string),
            Operand::Expression(expr) => expr.to_string(),
            Operand::None => "".to_string(),
            _ => format!("{:?}", *self).to_lowercase().to_string(),
        }
    }
}
