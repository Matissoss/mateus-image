#!/bin/python3

import getpass      as gp
import os           as os
import shutil       as sh
import subprocess   as sp

USER        = gp.getuser()

PROJ_NAME   = "mateus-image"
BUILD_DIR   = "build"

RUST_TARGETS    = ["x86_64-unknown-linux-gnu", "x86_64-unknown-linux-musl", "x86_64-pc-windows-gnu"]
SEPARATOR       = "-------------------------------------------------------------------"

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
    i = 0
    opts_len = len(opts)
    for opt in opts:
        if i+1 == opts_len:
            opt_string += opt
        else:
            opt_string += opt + "/"
        i += 1
    input_format = "{} [{}]\n> ".format(text,opt_string)
    input_string = input(input_format)
    for opt in opts:
        if opt == input_string:
            return opt
    return None

def build_dev():
    if os.path.exists(BUILD_DIR):
        sh.rmtree(BUILD_DIR)
    os.makedirs(BUILD_DIR)
    os.makedirs("{}/release".format(BUILD_DIR))
    for target in RUST_TARGETS:
        try:
            os.makedirs("{}/{}".format(BUILD_DIR,target))
            sp.run(["cargo", "build", "--release", "--target", target])
            if target == "x86_64-pc-windows-gnu":
                sh.move("target/{}/release/{}.exe".format(target,PROJ_NAME), "{}/{}/{}.exe".format(BUILD_DIR,target,PROJ_NAME))
            else:
                sh.move("target/{}/release/{}".format(target, PROJ_NAME), "{}/{}/{}".format(BUILD_DIR,target, PROJ_NAME))
            sp.run(["tar", "-czf", "build/release/{}.tar.gz".format(target), "{}/{}".format(BUILD_DIR,target)])
            print(SEPARATOR)
        except:
            print("Couldn't build for target {}".format(target))
            print(SEPARATOR)

def build_local():
    sp.run(["cargo", "build", "--release"])
    print(SEPARATOR)
    if allowed("Do you want to put executable in ~/.local/bin?\n> ".format(PROJ_NAME),'y','n'):
        try:
            sh.move("target/release/{}".format(PROJ_NAME), "/home/{}/.local/bin/{}".format(USER,PROJ_NAME))
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
        print(SEPARATOR)
        build_local()
    elif opt == 'd':
        print(SEPARATOR)
        build_dev()
    else:
        print("aborting build process!")
main()
