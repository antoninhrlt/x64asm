// This file is part of "x64asm"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

/// Creates an `Operand::DDirective`.
#[macro_export]
macro_rules! ddirective {
    ($ddirective:expr) => {
        instruction::Operand::DDirective($ddirective)
    }
}

/// "DDirective" stands for "Define Directive"
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DDirective {
    Db,
    Dw,
    Dd,
    Dq,
    Dt,
    Equ,
}

/// Convert the enum object identifier to a string as lowercase
impl ToString for DDirective {
    fn to_string(&self) -> String {
        format!("{:?}", self).to_lowercase()
    }
}
