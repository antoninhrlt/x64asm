// This file is part of "x64asm"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use crate::convert::{ ToAssembly, Separator };

/// Creates a `Mnemonic::Section`.
#[macro_export]
macro_rules! section {
    ($section:expr) => {
        instruction::Mnemonic::Section($section)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Section {
    Text,
    Bss,
    Data,
    Rodata,
}

impl ToAssembly for Section {
    fn to_assembly(&self, separator: Separator) -> String {
        match self {
            _ => format!("section .{:?}", self).to_lowercase()
        }
    }
}
