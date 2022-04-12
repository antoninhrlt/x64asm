// This file is part of "x64asm"
// Under the MIT License
// Copyright (c) 2022 Antonin Hérault

use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

use crate::instruction::Instruction;

/// Transform `Instruction`s to ASM code as String
pub struct Formatter {
    instructions: Vec<Instruction>,
    with_tabs: bool,
}

impl Formatter {
    pub fn new(with_tabs: bool) -> Self {
        Self {
            instructions: vec![],
            with_tabs,
        }
    }

    /// Add a new instruction to be formatted \
    /// Function designed to be followed by another method
    pub fn add_instruction(&mut self, instruction: Instruction) -> &mut Self {
        self.instructions.push(instruction);
        self
    }

    /// Add instructions to be formatted \
    /// Function designed to be followed by another method
    pub fn add_instructions(&mut self, instructions: &mut Vec<Instruction>) -> &mut Self {
        self.instructions.append(instructions);
        self
    }

    /// Main function to format the instructions to ASM code
    pub fn fmt(&self) -> String {
        let mut formatted = String::new();
        for instruction in &self.instructions {
            formatted += &instruction.to_string(self.with_tabs);
            formatted += &"\n";
        }

        formatted
    }

    /// Push the formatted content into a file and erase the content before
    pub fn to_file(&self, path: &Path) -> Result<(), std::io::Error> {
        let mut stream = File::create(path)?;
        stream.write_all(self.fmt().as_bytes())
    }

    /// Append the formatted content to the file's content
    pub fn append_to_file(&self, path: &Path) -> Result<(), std::io::Error> {
        let mut stream = File::options().append(true).open(path)?;
        stream.write_all(self.fmt().as_bytes())
    }
}
