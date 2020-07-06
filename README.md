# Super Units
Handle byte units easily

## Usage

```rust
extern crate super_units;

use super_units::Amount;

fn main() {
    let amount = Amount::auto_detect(32_f64 * 1024_f64);
    println!("{}", amount); // 32.0 Kb
    println!("{}", amount.bytes()); // 32768.0
    println!("{}", amount.quantity()); // 32.0
    println!("{}", amount.unit()); // Kb
}
```
