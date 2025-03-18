#!/bin/python3

import getpass
import os
import shutil
import subprocess as sp

RUST_TARGETS = ["x86_64-unknown-linux-gnu", "x86_64-unknown-linux-musl", "x86_64-pc-windows-gnu"]
USER = getpass.getuser()
PROJ_NAME = "mateus-image"
BUILD_DIR = "build"

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
        except:
            print("Couldn't build for target {}".format(target))

def build_local():
    if os.path.exists(BUILD_DIR):
        shutil.rmtree(BUILD_DIR)
    os.makedirs(BUILD_DIR)
    os.makedirs("{}/release".format(BUILD_DIR))
    sp.run(["cargo", "build", "--release"])
    print("Do you want to put {} binary in ~/.local/bin? (works only on Linux/XDG compatible path system)".format(PROJ_NAME))
    c = input("[y/n]: ")
    if c == "y":
        if not os.path.exists("/home/{}/.local/bin".format(USER)):
            print("~/.local/bin doesn't exists")
        else:
            shutil.move("target/release/mateus-image", "/home/{}/.local/bin/mateus-image".format(USER))
    print("done")

def main():
    print("Welcome to {}'s building script".format(PROJ_NAME))
    print("Choose build option: local (l) or dev (d) [recommended local]")
    choise = input("> ")
    if choise == "l":
        print("starting local build")
        build_local()
    elif choise == "d":
        print("starting global build")
        build_dev()
    else:
        print("aborting build")
main()
