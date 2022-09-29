import xml.etree.ElementTree as ET
import semver
import os.path
import shutil

output_folder_name = "pal_crates"
m0_folder_name = "svd_cm0"

def get_core_type(svd_file_path):
    """
    This function returns the core type
    :param 1 : svd_file_path
    :return: core type
    """
    tree = ET.parse(svd_file_path)
    root = tree.getroot()
    return root.find(r"./cpu/name").text


def create_m0_svd_file(original_svd_file,svd_output_path):
    """
    This function creates the svd files for cm0 based on the svd files of cm4/cm7 svd files
    :param 1 : original_svd_file(cm4/cm7 svd files)
    :param 2 : svd_output_path(path where it has to be created)
    :return: none
    """
    tree = ET.parse(original_svd_file)
    root = tree.getroot()
    root.find(r"./cpu/name").text = "M0"
    root.find(r"./cpu/fpuPresent").text = "false"
    root.find(r"./cpu/nvicPrioBits").text = "2"
    tree.write(svd_output_path)

def get_svd_version(svd_file_path):
    """
    This function returns the svd version from the svd files
    :param 1 : svd_file_path
    :return: returns the semantic version of svd
    """
    tree = ET.parse(svd_file_path)
    root = tree.getroot()
    svd_ver=root.find(r"./version").text
    # Parse optional because SVD generated doesn't follow semver pragma

    if not semver.VersionInfo.isvalid(svd_ver):
        print(f"generate_svd_crates: Try repairing invalid semversion {svd_ver}")
        svd_ver = svd_ver+".0"
    sem_ver = semver.VersionInfo.parse(svd_ver)
    # back to text
    return str(sem_ver)

def get_crate_name_without_core_revision(crate_name):
    """
    This function removes the revision and core names from the crates
    :param 1 : crate_name(e.g cyt2b9_a_cm0)
    :return: crate name without revision and core (e.g: cyt2b9_a_cm0 --> cyt2b9)
    """
    pos = crate_name.index('_')
    try:
        return crate_name[:crate_name.index('_', pos + 1)]
    except ValueError:
        return crate_name[:pos]

def remove_folder(name):
    """
    This function deletes the folder
    :param 1 : name(folder to be deleted)
    """
    if os.path.exists(name):
        shutil.rmtree(name)

def remove_files(folder, files):
    """
    This function deletes the list of files 
    :param 1 : name of the folder
    :param 2 : list of files to be deleted
    """
    if os.path.exists(folder):
        for file in files: 
            os.remove(folder+"\\"+file)

def move_files(src_folder,dst_folder, files):
    """
    This function deletes the list of files 
    :param 1 : name of the src folder
    :param 2 : name of the dst folder
    :param 3 : list of files to be moved
    """
    if os.path.exists(src_folder) and os.path.exists(dst_folder)  :
        for file in files: 
            shutil.move(src_folder+"\\"+file,dst_folder+"\\"+file)

def replace_text(file_name,search_text,replace_text):
    """
    This function replaces the text with new_text
    :param 1 : file to be modified
    :param 2 : search_text, current text content
    :param 3 : replace_text, text to be replace
    """
    if os.path.exists(file_name) :
        with open(file_name, 'r') as file:
            data = file.read()
            data = data.replace(search_text, replace_text)
  
        with open(file_name, 'w') as file:
            file.write(data)

def insert_text_in_between(file_name, search_text, insert_text):
    """
    This function inserts the text in between
    :param 1 : file to be modified
    :param 2 : search_text, line before insertion
    :param 3 : replace_text, lines to insert
    """
    location_of_line = 0
    with open(file_name, 'r') as file:
        data = file.read()
        lines = list(data.split(";"))
        #lines.insert(lines.index(search_text),insert_text)
        lines=list(map(lambda x: x.replace(search_text,insert_text+search_text),lines))
        build_rs_text = ' ; '.join(str(l) for l in lines)
    with open(file_name, 'w') as file:
            file.write(build_rs_text)

def write_to_file(file_name, file_contents):
    """
    This function writes to file 
    :param 1 : file to be modified
    :param 2 : file_contents to be added
    :return  : none
    """
    with open(file_name, "w") as file:
        file.write(file_contents)