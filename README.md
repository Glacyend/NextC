# NextC

**A system programming language inspired by C, offering modern syntax, stronger type safety, zero-cost abstractions, and full low-level control.**

---

This project is licensed under either of [Apache License, Version 2.0](https://license-apache/) or [MIT](https://license-mit/) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this project by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

- Author (English name): Glacyend
- Author (Chinese name): 周沁彬 (Zhou Qinbin)
- Author (Chinese art name): 凌雪蓝冰 (Lingxue Lanbing)

## Brief Introduction

The core goal of NextC is to provide more modern syntax and stronger type safety while preserving the simplicity and high freedom of C. The NextC compiler is written in Rust and uses the LLVM backend.

> [!Warning]
>
> NextC is currently in the preliminary design phase, and the basic language documentation is still being written. Please keep an eye on changes to the NextC documentation and this `README.md`.

Current NextC version: 0.0.0 + 2026.08

For more about NextC, please refer to the NextC documentation.

### Core Philosophy

- **Strict Type Safety**: Arrays and pointers are seperated; implicit type conversions are not allowed; pointer-to-other-type conversions are prohibited; strict `const` semantics.
- **Safe Pointer Operations**: Pointer arithmetic is disallowed; array-to-pointer decay is not supported; runtime null pointer checks (in Debug mode).
- **Flexible Memory Management**: Choose between manual memory management like C, or enable the planned GC (introduced in version 2.0, which will disable the manual `delete` statement).
- **Runtime Checks for Undefined Behavior**: In Debug mode, array bounds checking, `union` correctness, and other checks are performed to catch undefined behavior.
- **Efficient and Lightweight**: Borrowing C's simplicity and high performance, source code is compiled directly to LLVM IR, with further optimizations performed by LLVM.
- **Modern Syntax**: Adopts modern syntax features such as type inference, fixed-width integer types, mandatory braces, and multi-pass compilation.
- **Modular Programming**: Supports multi-file programming via modules.
- **C Interoperability**: Supports C FFI, allowing the use of external C functions and libraries.
- **Compatibility with Low-Level Unsafe Operations**: Use `unsafe` blocks for unavoidable low-level operations such as pointer arithmetic and array pointer manipulation.

## NextC Compiler

The NextC compiler uses Rust for its frontend and LLVM as its backend. The Rust third-party library Inkwell provides Rust bindings for LLVM.

The NextC compiler transforms the source code into an AST through lexical analysis and syntactic parsing. It then performs semantic analysis on the AST to generate an annotated AST, which is subsequently translated into LLVM IR by the code generation module. Finally, the LLVM backend performs further optimization and translation.
