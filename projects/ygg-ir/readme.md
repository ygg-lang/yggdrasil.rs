## Yggdrasil Compiler Compiler

## Optimize Pass

By default the following pass will be executed:

- [Import](): Load definitions from other files
- [CIR](): Explicit conversion ignore rules
- [Inline](): Inline rules starting with `_`
- [DCE](): Dead code elimination
- [Refine](): Clear redundant nodes
- [Fusion](): Use ac automata or DFA to speed up matching

## Backend & Codegen

- [Rust]():
- [Railway]():
- [Kotlin]():