# x64asm
Library to write x64 Assembly code from Rust, more properly. Designed for the nasm assembler

## How to use
```rust                                 
let mut f = Formatter::new(false); // false : do not replace spaces by tabulations

f.add_instructions(&mut vec![
    instruction!(/* <mnemonic>, [operands, ...] */),
    // other instructions
]);
f.to_file(&Path::new("output.asm")).unwrap();
```

## Installation
In your "Cargo.toml" file :
```toml
[dependencies]
x64asm = "*"
```

Check the current version on [crates.io](https://crates.io/crates/x64asm)

## Example
```rust      
let mut x64asm_formatter = Formatter::new(false);

x64asm_formatter.add_instructions(&mut vec![
    i!(Global, Op::Label("_start".to_string())),

    i!(section!(Text)),
    i!(label!("_start")),
    i!(Mov, reg!(Rax), Op::Literal(1)),
    i!(Mov, reg!(Rdi), Op::Literal(1)),
    i!(Mov, reg!(Rsi), Op::Label("msg".to_string())),
    i!(Mov, reg!(Rdx), Op::Label("msg_len".to_string())),
    i!(Syscall),

    i!(Mov, reg!(Rax), Op::Literal(60)),
    i!(Mov, reg!(Rdi), Op::Literal(0)),
    i!(Syscall),

    i!(section!(Data)),
    i!(label!("msg"), ddirective!(Db), Op::String("Hello world".to_string())),
    i!(label!("msg_len"), ddirective!(Equ), Op::Expression("$ - msg".to_string())),
]);

x64asm_formatter.to_file(&Path::new("example.asm")).unwrap();
``` 

<details>
<summary>Imports for the example</summary>

```rust
use x64asm::{
    ddirective, ddirective::DefineDirective::*, 
    formatter::Formatter, 
    instruction as i, label,
    mnemonic::Mnemonic::*, 
    operand::Op, 
    reg, register::Register::*, 
    section, section::Section::*,
};
```
</details>

And then, the generated "output.asm" file :
```asm
	global _start 
section .text 
_start: 
	mov rax, 1 
	mov rdi, 1 
	mov rsi, msg 
	mov rdx, msg_len 
	syscall 
	mov rax, 60 
	mov rdi, 0 
	syscall
section .data 
msg: db `Hello world` 
msg_len: equ $ - msg 
```

## Notes
**Inspired by** https://github.com/GregoryComer/rust-x86asm
