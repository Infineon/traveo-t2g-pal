# Peripheral access crate for T2G cyt2b9 

Revision = rev_c
svd version =  1.0.0
## Using device crate in your project

In your own project's `Cargo.toml`:
```toml
[dependencies.cyt2b9_c]
version = "0.0.1"
features = ["rt", "critical-section"]
```

The `rt` feature is optional.
See [svd2rust](https://docs.rs/svd2rust/latest/svd2rust/#the-rt-feature) for details.

The `critical-section` feature is optional.
See [critical-section](https://docs.rs/critical-section/latest/critical_section/) for details.

## Usage of device crate in your code:

```rust
use cyt2b9_c as pac;

let mut peripherals = pac::Peripherals::take().unwrap();
```
## Resource
[Documents](https://www.infineon.com/cms/en/product/microcontroller/32-bit-traveo-t2g-arm-cortex-microcontroller/#documents)

