#!/bin/python3

import getpass
import os
import shutil
import subprocess as sp

RUST_TARGETS = ["x86_64-unknown-linux-gnu", "x86_64-unknown-linux-musl", "x86_64-pc-windows-gnu"]
USER = getpass.getuser()
PROJ_NAME = "mateus-image"
BUILD_DIR = "build"

def allowed(text, yes, no) -> bool:
    input_string = input("[{}/{}]: {}".format(yes,no,text))
    if input_string == yes:
        return True
    elif input_string == no:
        return False
    elif input_string == ".exit":
        return False
    else:
        print("Try again or exit with .exit")
        return allowed(text,yes,no)

def chosen(text, opts):
    opt_string = ""
    for opt in opts:
        opt_string += opt + "/"
    input_format = "{} [{}]\n> ".format(text,opt_string)
    input_string = input(input_format)
    for opt in opts:
        if opt == input_string:
            return opt
    return None

def build_dev():
    if os.path.exists(BUILD_DIR):
        shutil.rmtree(BUILD_DIR)
    os.makedirs(BUILD_DIR)
    os.makedirs("{}/release".format(BUILD_DIR))
    for target in RUST_TARGETS:
        try:
            os.makedirs("{}/{}".format(BUILD_DIR,target))
            sp.run(["cargo", "build", "--release", "--target", target])
            shutil.move("target/{}/release/{}".format(target, PROJ_NAME), "{}/{}/{}".format(BUILD_DIR,target, PROJ_NAME))
            sp.run(["tar", "-czf", "build/release/{}.tar.gz".format(target), "{}/{}".format(BUILD_DIR,target)])
            print("-------------------------------------------------------------------")
        except:
            print("Couldn't build for target {}".format(target))
            print("-------------------------------------------------------------------")

def build_local():
    sp.run(["cargo", "build", "--release"])
    print("-------------------------------------------------------------------")
    if allowed("Do you want to put executable in ~/.local/bin? (recommended on Linux)\n> ".format(PROJ_NAME),'y','n'):
        try:
            shutil.move("target/release/{}".format(PROJ_NAME), "/home/{}/.local/bin/{}".format(USER,PROJ_NAME))
            print("Sucessfully done!")
        except:
            print("Something went wrong...\nTip: check if you have .local/bin directory in your home directory and if something still doesn't work then open a issue on github")
    else:
        print("Binary can be found in ./target/release path")

def main():
    print("===================================================================")
    print("#   #  ###  ##### ##### #   #  ####       # #   #  ###  ##### #####")
    print("## ## #   #   #   #     #   # #           # ## ## #   # #     #    ")
    print("# # # #   #   #   ##### #   #  ###  ##### # # # # #   # #  ## #####")
    print("#   # #####   #   #     #   #     #       # #   # ##### #   # #    ")
    print("#   # #   #   #   #####  ###  ####        # #   # #   # ##### #####")
    print("===================================================================")
    opt = chosen("Do you want local build or a dev build? (local build reccomended)", ['l','d'])
    if opt == 'l':
        print("-------------------------------------------------------------------")
        build_local()
    elif opt == 'd':
        print("-------------------------------------------------------------------")
        build_dev()
    else:
        print("aborting build process!")
main()
