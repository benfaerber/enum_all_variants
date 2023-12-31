# enum_all_variants [![](https://img.shields.io/crates/v/enum_all_variants.svg?logo=rust)](https://crates.io/crates/enum_all_variants)

Proc macro for generating a list of all enum variants.

## Example
```rust
use enum_all_variants::AllVariants;

#[derive(AllVariants, Debug)]
enum Direction {
    Left,
    Top,
    Right,
    Bottom,
}

fn main() {
    println!("{:?}", Direction::all_variants());
}
```

Outputs:
```
[Left, Top, Right, Bottom]
```
