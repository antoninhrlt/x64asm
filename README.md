# x64asm
Library to write x64 Assembly code from Rust, more properly. Designed for the nasm assembler

## How to use
```rust                                 
let instructions = vec![
    i!(/* <mnemonic>, [operands, ...] */),
    // other instructions
];

let code = instructions.to_assembly(/*separator (space or tab)*/);
// Writes to a file
let mut stream = File::create(&Path::new("output.asm")).unwrap();
write!(stream, "{}", code).unwrap();
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
let instructions = vec![
    i!(Global, oplabel!("_start")),

    i!(section!(Text)),
    i!(label!("_start")),
    i!(Mov, reg!(Rax), Op::Literal(1)),
    i!(Mov, reg!(Rdi), Op::Literal(1)),
    i!(Mov, reg!(Rsi), oplabel!("msg")),
    i!(Mov, reg!(Rdx), oplabel!("msg_len")),
    i!(Syscall),

    i!(Mov, reg!(Rax), Op::Literal(60)),
    i!(Mov, reg!(Rdi), Op::Literal(0)),
    i!(Syscall),
    
    i!(section!(Data)),
    i!(label!("msg"), dd!(Db), opstring!("Hello world")),
    i!(label!("msg_len"), dd!(Equ), opexpr!("$ - msg")),
];

let code = instructions.to_assembly(Separator::Space);
let mut stream = File::create(&Path::new("output.asm")).unwrap();
write!(stream, "{}", code).unwrap();
``` 

<details>
<summary>Imports for the example</summary>

```rust
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

use x64asm::convert::{ ToAssembly, Separator };
use x64asm::macros::*;
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
Originally inspired by [GregoryComer/rust-x86asm](https://github.com/GregoryComer/rust-x86asm).
