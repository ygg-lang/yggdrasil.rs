# Yggdrasil Generative Grammar

[![Build/test](https://github.com/tree-sitter/tree-sitter-rust/actions/workflows/ci.yml/badge.svg)](https://github.com/tree-sitter/tree-sitter-rust/actions/workflows/ci.yml)

Yggdrasil grammar DSL for [tree-sitter](https://github.com/tree-sitter/tree-sitter)

## Features

## Language Tutorial

- basic syntax

|    Name    | Description                  |
|:----------:|------------------------------|
|    `a?`    | Optional element             |
|    `a*`    | Zero or more elements        |
|    `a+`    | One or more elements         |
|   `a b`    | Sequence of elements         |
|  `a \| b`  | Alternative of branch        |
| `name: e`  | Mark element with given name |
|  `^rule`   | Unmark element               |
|  `#Name`   | Mark branch name             |      
| `@macro()` | Macro call                   |        
| `IGNORED`  | All rules marked as ignored  |
|   `ANY`    | All unicode characters       |

- `class` vs `union`

The same syntax `A | B` performs differently in `class` and `union` context.

```yggdrasil
// expand `A | B` as class
class TestA {
    | tag_a:A 
    | tag_b:B
}
// expand `A | B` as union
union TestB {
    | A #BranchA
    | B #BranchB
}
```


```rust
struct TestA {
    tag_a: A,
    tag_b: B,
}

enum TestB {
    BranchA(A),
    BranchB(B),
}
```

- examples

You can learn more from [examples](./examples).