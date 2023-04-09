# Yggdrasil Generative Grammar

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
|  `name:e`  | Mark element with given name |
|  `#Name`   | Mark branch with given name  |      
|  `^rule`   | Remark element               |
| `@macro()` | Macro call                   |        
|   `ANY`    | Any unicode characters       |
| `IGNORED`  | All rules marked as ignored  |

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

You can learn more from [project-yggdrasil](https://github.com/ygg-lang/project-yggdrasil/tree/master/languages).

## Tools

- [JetBrains Support](https://plugins.jetbrains.com/plugin/20594-yggdrasil-support)