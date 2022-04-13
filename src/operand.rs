// This file is part of "x64asm"
// Under the MIT License
// Copyright (c) 2022 Antonin Hérault

use crate::{
    register::Register,
    ddirective::DefineDirective,
};

pub type Op = Operand;

#[derive(Debug, PartialEq, Eq)]
pub enum Operand {
    /// Think to use `use x64asm::register::Register::*;`
    Register(Register),
    Indirect(Register),
    Literal(i64),
    DefineDirective(DefineDirective),
    Label(String),
    String(String),

    Expression(String),

    Byte,
    Word,
    Dword,
    Qword,

    None, // to avoid the `Option` usage
}

pub fn operand_vec_to_string(vec: &Vec<Operand>) -> String {
    let mut formatted = String::new();

    let mut is_variable = false;

    for (i, operand) in vec.iter().enumerate() {
        formatted += &format!("{}", operand.to_string());

        if is_variable && i != vec.len() - 1 {
            formatted += ",";
        }
        if i == 0 {
            match operand {
                Operand::DefineDirective(_) => is_variable = true,
                _ => if vec.len() != 1 { formatted += "," }, 
            }
        }
        formatted += " ";
    }
    formatted
}

impl ToString for Operand {
    fn to_string(&self) -> String {
        match &*self {
            Operand::Register(register) => register.to_string(),
            Operand::Indirect(register) => format!("[{}]", register.to_string()),
            Operand::Literal(value) => value.to_string(),
            Operand::DefineDirective(ddirective) => ddirective.to_string(),
            Operand::Label(label) => label.to_string(),
            Operand::String(string) => format!("`{}`", string),

            Operand::Expression(expression) => expression.to_string(),

            _ => format!("{:?}", *self).to_string(),
        }
    }
}
