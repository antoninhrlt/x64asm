// This file is part of "x64asm"
// Under the MIT License
// Copyright (c) 2022 Antonin Hérault

#[derive(Debug, PartialEq, Eq)]
pub enum DefineDirective {
    Db,
    Dw,
    Dd,
    Dq,
    Dt,
}

/// Convert the enum object identifier to a string as lowercase
impl ToString for DefineDirective {
    fn to_string(&self) -> String {
        format!("{:?}", self).to_lowercase()
    }
}
