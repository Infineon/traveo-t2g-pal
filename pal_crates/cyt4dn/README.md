## Supported Device Family
`cyt4dn`

## Supported Crates

|    crate      |  svd version  |
| ------------- | ------------- |
|    cyt4dn_a     |  1.0.0 |
|    cyt4dn_b     |  1.0.0 |
|    cyt4dn_c     |  1.0.0 |

## Using device meta-package crate in your project

In your own project's `Cargo.toml`:
```toml
[dependencies.cyt4dn]
version = "0.0.1"
features = ["<one of the supported revisions>", "rt", "critical-section"]
```

The `rt` feature is optional.
See [svd2rust](https://docs.rs/svd2rust/latest/svd2rust/#the-rt-feature) for details.

The `critical-section` feature is optional.
See [critical-section](https://docs.rs/critical-section/latest/critical_section/) for details.

Usage of device meta-package in your code:

```rust
use cyt4dn as pac;

let mut peripherals = pac::Peripherals::take().unwrap();
```
By default cyt4dn refers to the latest revision of the supported crates, if your project requires a specific revision, you can pass it as a feature in your project's cargo.toml file.
This way of using meta package you dont have to change your project code each time you change the revision.

## Resource
[Documents](https://www.infineon.com/cms/en/product/microcontroller/32-bit-traveo-t2g-arm-cortex-microcontroller/#documents)

