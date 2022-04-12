// This file is part of "x64asm"
// Under the MIT License
// Copyright (c) 2022 Antonin Hérault

use std::path::Path;

use x64asm::formatter::Formatter;
use x64asm::instruction::Instruction;
use x64asm::mnemonic::Mnemonic;
use x64asm::operand::Operand;

use x64asm::register::Register::*;

fn main() {
    let mut x64asm_formatter = Formatter::new(true);
    x64asm_formatter.add_instructions(&mut vec![
        Instruction::new(Mnemonic::Mov),
        Instruction::new_1(Mnemonic::Mov, Operand::Register(Rax))     
    ]);

    x64asm_formatter.to_file(&Path::new(&"test.asm"))
        .unwrap();
}
