// This file is part of "x64asm"
// Under the MIT License
// Copyright (c) 2022 Antonin Hérault

use std::string::ToString;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Section {
    Text,
    Bss,
    Data,
    Rodata,
}

#[macro_export]
macro_rules! section {
    ($section:expr) => {
        x64asm::mnemonic::Mnemonic::Section($section)
    }
}

/// Convert the enum object identifier to a string as lowercase
impl ToString for Section {
    fn to_string(&self) -> String {
        match self {
            _ => format!("section .{:?}", self).to_lowercase()
        }
    }
}
