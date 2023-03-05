import glob
import os.path
import os,time
import shutil
import re
from concurrent.futures import ProcessPoolExecutor,ALL_COMPLETED
import concurrent.futures
import file_templates
import util
import filecmp
import argparse
from subprocess import check_call
from configparser import ConfigParser


library_location ="local_file_system"
parser = argparse.ArgumentParser(description='parser to fetch type of crate to be generated')
parser.add_argument("--generate", 
                    choices=["generate_rev_specific_crates", "generate_meta_package"],
                    required=True, type=str, help="Crates generation type : either rev_specific_crates/ meta_package_crate")
args = parser.parse_known_args()
generate_type = args[0].generate

#for generating the meta package crates e.g cyt2b9 we need the revision specific crates e.g cyt2b9_a, this argument gives the location from where the libraries can be fetched
if generate_type == 'generate_meta_package':
     option_parser = argparse.ArgumentParser(description='parser to fetch the location of device specific crates')
     option_parser.add_argument("--library_location", 
                    choices=["infineon_artifactory", "cratesio", "local_file_system"], 
                    required=True, type=str, help="Location where the dependent revision specific libraries are available for the meta_package_crate")
     option_args = option_parser.parse_known_args()
     library_location = option_args[0].library_location

    

config_input_parser = ConfigParser()
config_input_parser.read('config_input.ini')
config_output_parser = ConfigParser()


core2target = {"CM4": "thumbv7em-none-eabihf",
               "CM7": "thumbv7em-none-eabihf",
               "CM0" : "thumbv6m-none-eabi"
               }


def get_crates_db():
    """
    This function creates the crates database.
    It is dictionary of crate name as key and svd file path as value
    :param : none
    :return: crates database
    """
    svd_files_paths = glob.glob("**/*.svd", recursive=True)
    result = {}
    for path in svd_files_paths:
        base_crate_name = os.path.splitext(os.path.basename(path))[0]
        folder_name = os.path.basename(os.path.dirname(path))
        revision_name = re.fullmatch(r"rev_(.*)", folder_name)[1]
        base_crate_name +=  '_' + revision_name
        core_name = util.get_core_type(path)
        result[base_crate_name] ={}
        result[base_crate_name] [core_name.lower()] = os.path.abspath(path)
        crate_name_m0 = base_crate_name + "_cm0"
        create_m0_svd_file_path = os.path.abspath(os.path.join(util.m0_folder_name, crate_name_m0))
        util.create_m0_svd_file(path,create_m0_svd_file_path)
        result[base_crate_name] ["cm0"] = os.path.abspath(os.path.join(util.m0_folder_name, crate_name_m0))
    return result


def get_meta_package_crates_db(crates_db):
    """
    This function creates the meta package database.
    e.g: A dictionary of cyt2b9 and all supported cores and revisions
    similarly for other devices
    :param1 : crates_db
    :return: meta_package database
    """
    crates_details = {"revs","version"}
    meta_packages =  dict([(device.split('_')[0],dict([(detail,[]) for detail in crates_details])) for device in crates_db])

    for device in crates_db:
        package_name = device.split('_')[0]
        revision = device.split('_')[1]
        version = config_input_parser.get(package_name,'new_ver')
        if version not in meta_packages[package_name]['version']:
            meta_packages[package_name]['version'].append(version)
        if revision not in meta_packages[package_name]['revs']:
          meta_packages[package_name]['revs'].append(revision)
    return meta_packages

def generate_meta_package_crate(package_name, package_details):
    """
    This function creates the meta package crate in the <util.output_folder_name> folder.
    It run rustfmt with 'cargo fmt'
    It checks the local package and all of its dependencies for errors using 'cargo check'
    :param 1 : meta_package name e.g:cyt2b9
    :param 2 : meta_package details, i.e all revisions supported for each device
    :return: none
    """
    util.remove_folder(package_name)# remove existing folder.
    os.mkdir(package_name)
    os.chdir(package_name)
    util.write_to_file("Cargo.toml",file_templates.get_meta_package_cargo_toml_text(package_name,package_details,config_input_parser,library_location))
    util.write_to_file("LICENSE.txt",file_templates.get_license_text())
    os.mkdir("src")
    os.chdir("src")
    util.write_to_file("lib.rs",file_templates.get_meta_package_lib_text(package_name,package_details))
    os.chdir("..")
    util.write_to_file("README.md",file_templates.get_meta_package_readme_text(package_name,package_details,crates_db))
    check_call('cargo fmt')
    check_call('cargo check --target=thumbv7em-none-eabihf --target=thumbv6m-none-eabi  --all-features')
    check_call('cargo doc --target=thumbv7em-none-eabihf --target=thumbv6m-none-eabi --all-features --no-deps')
    util.remove_folder("target")
    time.sleep(3)
    os.chdir("../")

def generate_crate(crate_name,cores):
    """
    This function creates the crate in the <util.output_folder_name> folder for each revision of device svd folder.
    It run rustfmt with 'cargo fmt'
    It checks the local package and all of its dependencies for errors using 'cargo check'
    :param 1 : crate_name name e.g:cyt2b9_a_cm0
    :param 2 : supported cores
    :return: none
    """
    os.mkdir(crate_name)
    os.chdir(crate_name)
    os.mkdir("src")
    os.chdir("src")
    for core in cores:
        os.mkdir(core[1:])
        os.chdir(core[1:])
        check_call(f'svd2rust.exe -i {cores[core]} -o "."')
        check_call('form -i lib.rs -o "."')
        os.rename('lib.rs', 'mod.rs')
        util.replace_text(os.getcwd()+"\\mod.rs", "use generic :: * ;" , "use crate::generic::*;")
        util.replace_text(os.getcwd()+"\\mod.rs", "# [doc = r\"Common register and bit access and modify traits\"] pub mod generic ;" , "")
        util.replace_text(os.getcwd()+"\\mod.rs", "# ! [no_std]" , "")
        os.chdir("..")

    curr_dir = os.getcwd()
    core_names = list(cores.keys())
    if not filecmp.cmp(curr_dir+'\\'+core_names[0][1:]+'\\generic.rs', curr_dir+'\\'+core_names[1][1:]+'\\generic.rs', shallow=False):
        raise Exception("generic.rs are not the same for both the cores")
    else:
        util.move_files(curr_dir+'\\'+core_names[0][1:], curr_dir,"generic.rs".split())
        common_files = ["build.rs", "device.x"]
        util.move_files(curr_dir+'\\'+core_names[0][1:], curr_dir+'\\..' ,common_files)
        remove_files = ["generic.rs","build.rs","device.x"]
        util.remove_files(curr_dir+'\\'+core_names[1][1:], remove_files)
        util.write_to_file("lib.rs",file_templates.get_crate_lib_text(crate_name,core_names))

        svd_version = util.get_svd_version(cores.get('cm0')) #svd version same for both cores
        os.chdir("..")
        before_line = "println ! (\"cargo:rerun-if-changed=build.rs\") "
        util.insert_text_in_between(os.getcwd()+"\\build.rs", before_line, file_templates.get_build_rs_conditional_flag_text(cores))
        version = config_input_parser.get(crate_name,'new_ver')
        revision = crate_name.split('_')[1]
        package_details = {
        'device' : crate_name.split('_')[0],
        'rev' : revision,
        'svd_version' : svd_version,
        'version' : version
        }
        util.write_to_file("Cargo.toml",file_templates.get_cargo_toml_text(crate_name,package_details))
        util.write_to_file("README.md",file_templates.get_package_readme_text(crate_name,package_details))
        util.write_to_file("LICENSE.txt",file_templates.get_license_text())
        check_call('cargo fmt')
        check_call('cargo check --target=thumbv7em-none-eabihf --target=thumbv6m-none-eabi  --all-features')
        check_call('cargo doc --target=thumbv7em-none-eabihf --target=thumbv6m-none-eabi --all-features --no-deps')
        shutil.rmtree("target")
        os.chdir("..")


def update_config_current_ver(meta_package_crates_db):
    """
    This function creates a new config_ouput.ini file in the <util.output_folder_name> folder
    The curr_ver of all crates are updated to the new_ver from the config_input.ini
    This config_ouput.ini can be reused as config_input.ini in the next release
    :param 1 : meta_package_crates_db
    :return: none
    """
    for package_name, package_details in meta_package_crates_db.items():
        config_output_parser.add_section(package_name)
        new_ver = config_input_parser.get(package_name, 'new_ver')
        config_output_parser.set(package_name,'curr_ver',new_ver)
        config_output_parser.set(package_name,'new_ver',new_ver)
        for rev in package_details["revs"]:
            crate_name = package_name+"_"+rev
            new_ver_crate = config_input_parser.get(crate_name, 'new_ver')
            svd_version = util.get_svd_version(crates_db[crate_name]["cm0"])#svd version same for both cores
            config_output_parser.add_section(crate_name)
            config_output_parser.set(crate_name,'svd_ver',svd_version)
            config_output_parser.set(crate_name,'curr_ver',new_ver_crate)
            config_output_parser.set(crate_name,'new_ver',new_ver_crate)

    with open("./config_output.ini", "w") as file_out:
        config_output_parser.write(file_out)



util.remove_folder(util.m0_folder_name)
os.mkdir(util.m0_folder_name)

crates_db = get_crates_db()
meta_package_crates_db = get_meta_package_crates_db(crates_db)


if generate_type == "generate_rev_specific_crates" :
    util.remove_folder(util.output_folder_name)
    os.mkdir(util.output_folder_name)
    os.chdir(util.output_folder_name)
    for k, v in crates_db.items():
        generate_crate(k, v)

#Generate meta packages only if the revision specific device crates are already available
if generate_type == "generate_meta_package" :
    if not os.path.exists(util.output_folder_name):
        raise Exception("First run the command poetry run 'python.exe generate_svd_crates.py --generate generate_rev_specific_crates' to generate the output folder with revision specific crates")
    else:
        os.chdir(util.output_folder_name) # assumption that output folder already exists after running python.exe generate_svd_crates.py --generate  generate_rev_specific_crates
        for k, v in meta_package_crates_db.items():
            generate_meta_package_crate(k, v)

update_config_current_ver(meta_package_crates_db)

# Remove svd for m0 generated folder
util.remove_folder(util.m0_folder_name)
