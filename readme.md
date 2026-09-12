```shell
# fail
cargo check

# work
cargo check -p bit32

# work
cargo check -p bit64


cargo tree --workspace -e features


cargo check -p exclusive_feature --no-default-features --features int_are_32_bits
cargo check -p exclusive_feature --no-default-features --features int_are_64_bits
```