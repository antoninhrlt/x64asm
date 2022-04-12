// This file is part of "x64asm"
// Under the MIT License
// Copyright (c) 2022 Antonin Hérault

use crate::register::Register;

#[derive(Debug, PartialEq, Eq)]
pub enum Operand {
    /// Think to use `use x64asm::register::Register::*;`
    Register(Register),

    None, // to avoid the `Option` usage
}

impl ToString for Operand {
    fn to_string(&self) -> String {
        match &*self {
            Operand::Register(register) => return register.to_string(),
            _ => {},
        }

        match *self {
            Operand::None => "", // should never happens
            _ => panic!(), // already covered, panic will never happen
        }
        .to_string()
    }
}
