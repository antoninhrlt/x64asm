// This file is part of "x64asm"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use crate::convert::{ ToAssembly, Separator };
use super::{ Label, Section };

/// Designed to aid the memory, it's like a symbol describing the operation, 
/// and it's followed by operands in the common situations.
///
/// Complete list here : https://cs.brown.edu/courses/cs033/docs/guides/x64_cheatsheet.pdf
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Mnemonic {
    Label(Label),
    Section(Section),
    Expression(String),

    Mov,
    Movzx,
    Movsx,
    Push,
    Pop,
    Cwtl,
    Cltq,
    Cqto,
    Inc,
    Neg,
    Not,
    Lea,
    Leaq,
    Add,
    Sub,
    Mul,
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
    Div,
    Divq,
    Idiv,
    Cmp,
    Test,
    Sete, 
    Setz,
    Setl,
    Setne, 
    Setnz,
    Sets, 
    Setns,
    Setg, 
    Setnle,
    Setge, 
    Setnl,
    Seta, 
    Setnbe,
    Setae, 
    Setnb,
    Setb, 
    Setnae,
    Setbe, 
    Setna,
    Setle,
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
    Extern,
}

impl ToAssembly for Mnemonic {
    fn to_assembly(&self, separator: Separator) -> String {
        match self {
            Self::Label(label) => label.to_assembly(separator),
            Self::Section(section) => section.to_assembly(separator),
            Self::Expression(string) => string.clone(),
            _ => format!("{:?}", self).to_lowercase()
        }
    }
}
