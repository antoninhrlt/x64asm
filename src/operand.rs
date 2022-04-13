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
                _ => formatted += ",", 
            }
        }
        formatted += " ";
    }
    formatted
}

impl ToString for Operand {
    fn to_string(&self) -> String {
        match &*self {
            Operand::Register(register) => return register.to_string(),
            Operand::Indirect(register) => return format!("[{}]", register.to_string()),
            Operand::Literal(value) => return value.to_string(),
            Operand::DefineDirective(ddirective) => return ddirective.to_string(),
            
            Operand::Expression(string) => return string.to_string(),

            _ => {},
        }

        match *self {
            Operand::Byte => "byte",
            Operand::Word => "word",
            Operand::Dword => "dword",
            Operand::Qword => "qowrd",
            Operand::None => "", // should never happens
            _ => panic!(), // already covered, panic will never happen
        }
        .to_string()
    }
}
