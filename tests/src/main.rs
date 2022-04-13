// This file is part of "x64asm"
// Under the MIT License
// Copyright (c) 2022 Antonin Hérault

use std::path::Path;

use x64asm::{
    instruction, 
    reg,
    indirect_reg,
    label,
    ddirective::DefineDirective,
    formatter::Formatter,
    instruction::Instruction,
    label::Label,
    mnemonic::Mnemonic,
    operand::Op,
    register::Register,
};

fn main() {
    let mut x64asm_formatter = Formatter::new(false);
    x64asm_formatter.add_instructions(&mut vec![
        instruction!(label!("salut"), Op::DefineDirective(DefineDirective::Dw), Op::Literal(5)),
        instruction!(Mnemonic::Xor, reg!(Register::Rax), reg!(Register::Rax)),
        instruction!(Mnemonic::Mov, reg!(Register::Rax), Op::Literal(289)),
        instruction!(Mnemonic::Mov, reg!(Register::Rbx), Op::Dword, Op::Indirect(Register::Rax)),
        instruction!(Mnemonic::Leave),
    ]);

    x64asm_formatter.to_file(&Path::new("test.asm")).unwrap();
}
