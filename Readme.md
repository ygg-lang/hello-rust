Yggdrasil Template
==================

## Build with CLI(Recommend)

1. Install `ycc`

```shell
# require rust nightly
cargo install ycc --force
```

or download from [release](https://github.com/ygg-lang/yggdrasil.rs/releases) and add to `$PATH`.

2. Written grammar definition file,
- Example: [json5.ygg](projects/build_by_dep/grammars/json5.ygg)

3. Run build command

```
ycc build
```


## Build with RUST dependence 

1. Written grammar definition file, 
  - Example: [json5.ygg](projects/build_by_dep/grammars/json5.ygg)

2. Define build dependencies in cargo(require rust nightly)

```toml
[build-dependencies.yggdrasil-shared]
version = "0.2.3"
# git = "https://github.com/ygg-lang/yggdrasil.rs"
# branch = "dev"
```

3. Write the `build.rs`

- See: [build.rs](projects/build_by_dep/build.rs)

4. Run `cargo build` and reexport needed symbols

- See: [lib.rs](projects/build_by_dep/src/lib.rs)

## Tools

- [Jetbrain Plugins](https://plugins.jetbrains.com/plugin/20594-yggdrasil-support)

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

## Community

- [Github Issue](https://github.com/ygg-lang/yggdrasil.rs/issues)
- [Github Discussion](https://github.com/ygg-lang/project-yggdrasil/discussions)
- [Discord](https://discord.gg/rDScD9GyUC)
