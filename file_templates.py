import util

def get_cargo_toml_text(package_name, package_details):
    """
    This function returns the contents of the cargo.toml file present in the crate
    :param 1 : package_name(crate name)
    :param 2 : package_details(crate details)
    :return: cargo.toml content in the form of text
    """
    version = (' '.join(package_details["svd_version"]))
    text= f"""[package]
name = "{package_name}"
version = \"{package_details["version"]}\"
authors = ["Infineon developers"]
edition = "2021"
rust-version = "1.64"
description = "Peripheral access crate for {package_name} T2G family"
documentation = "https://docs.rs/{package_name}"
repository = "https://github.com/Infineon/traveo-t2g-pal/tree/develop/{util.output_folder_name}/{package_name}"
license-file = "LICENSE.txt"
readme = "README.md"
keywords = ["cortex-m", "no_std", "traveo", "infineon"]
categories = ["embedded", "no-std"]

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
"""

    text+= f"""
[dependencies]
critical-section = {{ version = "1.0", optional = true }}
cortex-m = "0.7.6"
vcell = "0.1.2"

[dependencies.cortex-m-rt]
optional = true
version = "0.7"

[features]
rt = ["cortex-m-rt/device"]

[package.metadata.docs.rs]
"""

    text+="targets = [\"thumbv6m-none-eabi\" , \"thumbv7em-none-eabihf\"] \n"
    text+="""features = ["rt","critical-section"]"""
    return text


def get_meta_package_cargo_toml_text(package_name, package_details, config_input_parser,library_location):
    """
    This function returns the contents of the cargo.toml file present in the meta_package_crate
    :param 1 : package_name(meta_package_crate name)
    :param 2 : package_details(meta_package_crate details)
    :param 3 : config_input_parser(contents of config_input.ini file to fetch the version)
    :return: cargo.toml content in the form of text
    """
    version = (' '.join(package_details["version"]))
    text= f"""[package]
name = "{package_name}"
version = "{version}"
authors = ["Infineon developers"]
edition = "2021"
rust-version = "1.64"
description = "Peripheral access crate for {package_name} T2G family"
documentation = "https://docs.rs/{package_name}"
repository = "https://github.com/Infineon/traveo-t2g-pal/tree/develop/{util.output_folder_name}/{package_name}"
license-file = "LICENSE.txt"
readme = "README.md"
keywords = ["cortex-m", "no_std", "traveo", "infineon"]
categories = ["embedded", "no-std"]

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

"""
    text+="[dependencies] \n"
    for rev in package_details["revs"]:

        crate_version = config_input_parser.get(package_name+"_"+rev,'new_ver')
        if library_location == "local_file_system" :
            text+=f"{package_name}_{rev} = {{path = \"../{package_name}_{rev}\",optional = true}}\n"
        elif library_location == "infineon_artifactory" :
            text+=f"{package_name}_{rev} = {{version = \"{crate_version}\", registry = \"atvmrust\" ,optional = true}}\n"
        elif library_location == "cratesio" :
            text+=f"{package_name}_{rev} = {{version = \"{crate_version}\",optional = true}}\n"
        text+="\n"

    text+=f"""[features]
default = [\"rev_{package_details["revs"][-1]}\"]\n"""
    for rev in package_details["revs"]:
        text+=f"rev_{rev} = ["
        text+=f"\"{package_name}_{rev}\""
        text+=","
        text=text[:-1]+"]\n"
    text+="rt = ["
    for rev in package_details["revs"]:
        text+=f"\"{package_name}_{rev}?/rt\","
    text=text[:-1]+"]\n"
    text+="critical-section = ["
    for rev in package_details["revs"]:
        text+=f"\"{package_name}_{rev}?/critical-section\","
    text=text[:-1]+"]\n"
    text+="""
[package.metadata.docs.rs]
targets = ["thumbv6m-none-eabi","thumbv7em-none-eabihf"]
features = ["rt","critical-section",""" + ",".join(map(lambda x: f'"rev_{x}"',package_details["revs"])) + "]\n"
    return text

def get_crate_lib_text(package_name, core_names):
    """
    This function returns the contents of the lib.rs file present in the crate
    :param 1 : package_name(crate name)
    :param 2 : package_details(crate details)
    :return: lib.rs content in the form of text
    """
    text = f'''#![no_std]\n
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;
#[allow(unused_imports)]
use crate::generic::*;\n'''

    for core in core_names:
        text+=f"""#[cfg({core})]
mod {core[1:]};
#[cfg({core})]
pub use self::{core[1:]}::*;
"""
    return text

def get_build_rs_conditional_flag_text(core_names):
    """
    This function returns the conditional flag related text of build.rs
    :param 1 : core_names(supported cores)
    :return: text to be added
    """
    text = " let target = env::var_os(\"TARGET\").unwrap();"
    text += "match target.to_str().unwrap() {\n"
    for core in core_names:
        if core == "cm0":
            text+=f'"thumbv6m-none-eabi" => println!("cargo:rustc-cfg={core}"),\n'
        else:
            text+=f'"thumbv7em-none-eabihf" => println!("cargo:rustc-cfg={core}"),\n'
    text+="""_ => panic!("Only thumbv7em-none-eabihf and thumbv6m-none-eabi target triple are supported")
    }
    """
    return text

def get_meta_package_lib_text(package_name, package_details):
    """
    This function returns the contents of the lib.rs file present in the meta_package_crate
    :param 1 : package_name(meta_package_crate name)
    :param 2 : package_details(meta_package_crate details)
    :return: lib.rs content in the form of text
    """
    text = f"#![no_std]\n\n"
    for rev in package_details["revs"]:
        text+=f"""#[cfg(all(feature = \"rev_{rev}\"))]
pub use {package_name}_{rev}::*;
"""
    return text

def get_meta_package_readme_text(package_name, package_details, crates_db):
    """
    This function returns the contents of the README.md file present in the meta_package_crate
    :param 1 : package_name(meta_package_crate name)
    :param 2 : package_details(meta_package_crate details)
    :param 3 : crates_db(database of all crates)
    :return: README.md content in the form of text
    """
    version = (' '.join(package_details["version"]))
    text =f"""## Supported Device Family
`{package_name}`

## Supported Crates\n
|    crate      |  svd version  |
| ------------- | ------------- |\n"""

    for rev in package_details["revs"]:
        crate_name = package_name+"_"+rev
        svd_version = util.get_svd_version(crates_db[crate_name]["cm0"])#svd version same for both cores
        text+=f"|    {crate_name}     |  {svd_version} |\n"

    text=text[:-1]+"\n"
    text += f"""\n## Using device meta-package crate in your project

In your own project's `Cargo.toml`:
```toml
[dependencies.{package_name}]
version = "{version}"
features = ["<one of the supported revisions>", "rt", "critical-section"]
```

The `rt` feature is optional.
See [svd2rust](https://docs.rs/svd2rust/latest/svd2rust/#the-rt-feature) for details.

The `critical-section` feature is optional.
See [critical-section](https://docs.rs/critical-section/latest/critical_section/) for details.

Usage of device meta-package in your code:

```rust
use {package_name} as pac;

let mut peripherals = pac::Peripherals::take().unwrap();
```
By default {package_name} refers to the latest revision of the supported crates, if your project requires a specific revision, you can pass it as a feature in your project's cargo.toml file.
This way of using meta package you dont have to change your project code each time you change the revision.

## Resource
[Documents](https://www.infineon.com/cms/en/product/microcontroller/32-bit-traveo-t2g-arm-cortex-microcontroller/#documents)

"""
    return text

def get_package_readme_text(package_name, package_details):
    """
    This function returns the contents of the README.md file present in the crate that is created for each svd file
    :param 1 : package_name(crate name)
    :param 2 : package_details(crate details)
    :return: README.md content in the form of text
    """

    text = f"""# Peripheral access crate for T2G {package_details["device"]} \n\n"""

    text += f"""Revision = rev_{package_details["rev"]}

svd version =  {package_details["svd_version"]}\n"""

    text += f"""## Using device crate in your project

In your own project's `Cargo.toml`:
```toml
[dependencies.{package_name}]
version = \"{package_details["version"]}\"
features = ["rt", "critical-section"]
```

The `rt` feature is optional.
See [svd2rust](https://docs.rs/svd2rust/latest/svd2rust/#the-rt-feature) for details.

The `critical-section` feature is optional.
See [critical-section](https://docs.rs/critical-section/latest/critical_section/) for details.

## Usage of device crate in your code:

```rust
use {package_name} as pac;

let mut peripherals = pac::Peripherals::take().unwrap();
```
## Resource
[Documents](https://www.infineon.com/cms/en/product/microcontroller/32-bit-traveo-t2g-arm-cortex-microcontroller/#documents)

"""
    return text

def get_license_text() :
    """
    This function returns the contents of the LICENSE.txt file present in the crate
    :return: license file content in the form of text
    """
    text = f"""Copyright 2023 Infineon Technologies AG

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”),
to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense,
and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

"""
    return text
