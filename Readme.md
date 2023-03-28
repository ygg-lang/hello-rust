Yggdrasil Template
==================

## Build with CLI(Recommend)

1. Install `ycc`

```shell
cargo install ycc --force
```

2. Written grammar definition file,
- Example: [json5.ygg](projects/build_by_dep/grammars/json5.ygg)

3. Run build command

```
ycc build
```


## Build with RUST dependence 

1. Written grammar definition file, 
  - Example: [json5.ygg](projects/build_by_dep/grammars/json5.ygg)

2. Define build dependencies in cargo

- Require rust nightly

```toml
[build-dependencies.yggdrasil-shared]
version = "0.2.3"
```

3. Write the `build.rs`

- See: [build.rs](projects/build_by_dep/build.rs)

4. Run `cargo build` and reexport needed symbols

- See: [lib.rs](projects/build_by_dep/src/lib.rs)

## Tools

- [Jetbrain Plugins](https://plugins.jetbrains.com/plugin/20594-yggdrasil-support)

## Report Problem

- [Github Issue](https://github.com/ygg-lang/yggdrasil-rs/issues/new)