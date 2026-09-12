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

This behavior is due to [Feature Unification](https://doc.rust-lang.org/cargo/reference/features.html#mutually-exclusive-features), from the cargo reference :

> ### Mutually exclusive features
> There are rare cases where features may be mutually incompatible with one another. This should be avoided if at all possible, because it requires coordinating all uses of the package in the dependency graph to cooperate to avoid enabling them together. If it is not possible, consider adding a compile error to detect this scenario. For example:
> ```rs
> #[cfg(all(feature = "foo", feature = "bar"))]
> compile_error!("feature \"foo\" and feature \"bar\" cannot be enabled at the same time");
> ```
> Instead of using mutually exclusive features, consider some other options:
> 
> - Split the functionality into separate packages.
> - When there is a conflict, choose one feature over another. The cfg-if package can help with writing more complex cfg expressions.
> - Architect the code to allow the features to be enabled concurrently, and use runtime options to control which is used. For example, use a config file, command-line argument, or environment variable to choose which behavior to enable.

See also [this reddit post](<https://www.reddit.com/r/rust/comments/10uhpbm/workspace_with_two_binaries_accessing_different/>) for more info.

# Other draft / random cool command :

```shell
cargo tree --workspace -e features

cargo check -p exclusive_feature --no-default-features --features int_are_32_bits
cargo check -p exclusive_feature --no-default-features --features int_are_64_bits
```