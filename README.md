# T2G Peripheral Access Layer (PAL) generation demo

## Prerequisites

* Python 3.10 or greater
* poetry python package
  * Install using `pip install poetry`
* SVD2Rust version > 0.28.0
  * Install using `cargo install svd2rust`
* Form
  * Install using `cargo install form`
* rustfmt
  * Install using `rustup component add rustfmt`

## Generate or update T2G PAL crates

Execute python script:

```bash
poetry run  python.exe generate_svd_crates.py --generate <generate_rev_specific_crates | generate_meta_package> --library_location <required only if --generate = generate_meta_package:  infineon_artifactory | cratesio | local_file_system>
```

This script generates a revision specific crate and meta package crate for each device.
SVDs are read from `svd` folder and generates M0 version of svd.

### Supported Command line arguments

 *- -generate*
choices=["generate_rev_specific_crates", "generate_meta_package"] :
*"generate_rev_specific_crates"* -> generates revision specific crates for each device. The cm0 and cm4/cm7 combined into a single crate. 
Crate names are `<product_name>_<revision>` e.g crate name :cyt2b9_c
Execution command e.g: *poetry run  python.exe generate_svd_crates.py --generate generate_rev_specific_crates*

*"generate_meta_package"* -> Beside the revision specific crate, the script generates also a convenience crate for each device combination. This script reexport the api of core crates based on target architecture and selected revision. To be run only if the revision specific crates are already available. Crates names are `<product_name>` e.g crate name : cyt2b9 

*- -library_location* 
choices=["infineon_artifactory", "cratesio", "local_file_system"]
if *- -generate* is *generate_meta_package*, you have to also specify the location from where the revision specific crates are fetched with an additional argument *- -library_location* .
Execution command e.g: *poetry run  python.exe generate_svd_crates.py --generate generate_meta_package --library_location local_file_system*

### Supported targets

*CM0* = thumbv6m-none-eabi
*CM4/CM7* = thumbv7em-none-eabihf 

### Config files

*config_input.ini* file where new version of crates can be added
*config_output.ini* file has the up-to-date version of crates after running the script.
