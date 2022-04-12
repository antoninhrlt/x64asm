// This file is part of "x64asm"
// Under the MIT License
// Copyright (c) 2022 Antonin Hérault

use std::string::ToString;

/// Proper way to make a label for a function, a static object, ... with or 
/// without colon
#[derive(Debug, Eq, PartialEq)]
pub struct Label {
    label: String,
    is_colon: bool,
}

impl ToString for Label {
    fn to_string(&self) -> String {
        format!("{}{}", self.label, if self.is_colon {":"} else {""})
    }
}

impl Label {
    pub fn new(label: String) -> Self {
        Self {
            label,
            is_colon: true,
        }
    }

    pub fn with_no_colon(&mut self) {
        self.is_colon = false;
    }
}
