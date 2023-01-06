// This file is part of "x64asm"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

/// The character that separate the code elements in a line.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Separator {
    Space,
    //Spaces(usize),
    Tabulation,
    //Tabulations(usize),
}

impl Separator {
    pub fn value(&self) -> &str {
        match self {
            Self::Space => " ",
            Self::Tabulation => "\t",
        }
    } 
}
