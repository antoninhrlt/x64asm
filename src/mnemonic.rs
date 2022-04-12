// This file is part of "x64asm"
// Under the MIT License
// Copyright (c) 2022 Antonin Hérault

use std::string::ToString;

/// Also called as "instruction" (but keep as note that an instruction contains
/// the mnemonic + operands, it's just a simplified way to say that and I never
/// use this way to speak in this library), a mnemonic is designed to aid the 
/// memory, it's like a symbol describing the operation, and it's followed 
/// by operands in the common situations
///
/// Complete list here : https://cs.brown.edu/courses/cs033/docs/guides/x64_cheatsheet.pdf
#[derive(Debug)]
pub enum Mnemonic {
    Mov,
}

impl ToString for Mnemonic {
    fn to_string(&self) -> String {
        match *self {
            Mnemonic::Mov => "mov",
        }
        .to_string()
    }
}
