# Testing how exclusive feature work in a workspace

The crate `exclusive_feature` define some exclusive feature `int_are_32_bits` and `int_are_64_bits`,
and crate `bit32` depend on `exclusive_feature` with the feature `int_are_32_bits`, 
while crate `bit64` depend on `exclusive_feature` with the feature `int_are_64_bits`.

Command like 
```shell
# Fail to check the whole workspace
cargo check
```

But checking them individually work :

```shell
# Work
cargo check -p bit64
cargo check -p bit32
```

I don't think there is an unify way to call `cargo check` one time to check the whole workspace.

# Other draft / random cool command :

```shell
cargo tree --workspace -e features

cargo check -p exclusive_feature --no-default-features --features int_are_32_bits
cargo check -p exclusive_feature --no-default-features --features int_are_64_bits
```