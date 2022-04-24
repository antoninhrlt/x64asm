// This file is part of "x64asm"
// Under the MIT License
// Copyright (c) 2022 Antonin Hérault

use std::string::ToString;

use crate::label::Label;
use crate::section::Section;

/// Also called as "instruction" (but keep as note that an instruction contains
/// the mnemonic + operands, it's just a simplified way to say that and I never
/// use this way to speak in this library), a mnemonic is designed to aid the 
/// memory, it's like a symbol describing the operation, and it's followed 
/// by operands in the common situations
///
/// Complete list here : https://cs.brown.edu/courses/cs033/docs/guides/x64_cheatsheet.pdf
#[derive(Debug, Clone)]
pub enum Mnemonic {
    Label(Label),
    Section(Section),
    Expression(String),

    Mov,
    Push,
    Pop,
    Cwtl,
    Cltq,
    Cqto,
    Inc,
    Neg,
    Not,
    Leaq,
    Add,
    Sub,
    Imul,
    Xor,
    Or,
    And,
    Sal,
    Shl,
    Sar,
    Shr,
    Imulq,
    Mulq,
    Idivq,
    Divq,
    Cmp,
    Test,
    Sete, Setz,
    Setne, Setnz,
    Sets, 
    Setns,
    Setg, Setnle,
    Setge, Setnl,
    Seta, Setnbe,
    Setae, Setnb,
    Setb, Setnae,
    Setbe, Setna,
    Jmp,
    Je, Jz,
    Jne, Jnz,
    Js,
    Jns,
    Jg, Jnle,
    Jge, Jnl,
    Jl, Jnge,
    Jle, Jng,
    Ja, Jnbe,
    Jae, Jnb,
    Jb, Jnae,
    Jbe, Jna,
    Cmove, Cmovz,
    Cmovne, Cmovnz,
    Cmovs,
    Cmovns,
    Cmovg, Cmovnle,
    Cmovge, Cmovnl,
    Cmovl, Cmovnge,
    Cmovle, Cmovng,
    Cmova, Cmovnbe,
    Cmovae, Cmovnb,
    Cmovb, Cmovnae,
    Cmovbe, Cmovna,
    Call,
    Leave,
    Ret,
    Syscall,
    Global,
}

/// Convert the enum object identifier to a string as lowercase
impl ToString for Mnemonic {
    fn to_string(&self) -> String {
        match self {
            Mnemonic::Label(label) => label.to_string(),
            Mnemonic::Section(section) => section.to_string(),
            Mnemonic::Expression(string) => string.to_string(),
            _ => format!("{:?}", self).to_lowercase()
        }
    }
}
