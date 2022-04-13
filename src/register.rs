// This file is part of "x64asm"
// Under the MIT License
// Copyright (c) 2022 Antonin Hérault

pub type Reg = Register;

#[macro_export]
macro_rules! reg {
    ($register:expr) => {
        x64asm::operand::Operand::Register($register)
    }
}

#[macro_export]
macro_rules! indirect_reg {
    ($register:expr) => {
        x64asm::operand::Operand::Indirect($register)
    }
}

/// All registers available for x64 architecture (8 to 64 bits registers)
#[derive(Debug, PartialEq, Eq)]
pub enum Register {
    // 64 bits
    Rax,
    Rbx,
    Rcx,
    Rdx,
    Rsi,
    Rdi,
    Rbp,
    Rsp,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,

    // 32 bits
    Eax,
    Ebx,
    Ecx,
    Edx,
    Esi,
    Edi,
    Ebp,
    Esp,
    R8d,
    R9d,
    R10d,
    R11d,
    R12d,
    R13d,
    R14d,
    R15d,

    // 16 bits
    Ax,
    Bx,
    Cx,
    Dx,
    Si,
    Di,
    Bp,
    Sp,
    R8w,
    R9w,
    R10w,
    R11w,
    R12w,
    R13w,
    R14w,
    R15w,

    // 8 bits
    Al,
    Bl,
    Cl,
    Dl,
    Sil,
    Dil,
    Bpl,
    Spl,
    R8b,
    R9b,
    R10b,
    R11b,
    R12b,
    R13b,
    R14b,
    R15b
}

/// Convert the enum object identifier to a string as lowercase
impl ToString for Register {
    fn to_string(&self) -> String {
        format!("{:?}", self).to_lowercase()
    }
}
