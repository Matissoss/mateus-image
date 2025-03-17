#!/bin/python3

import getpass
import os
import shutil
import subprocess as sp

RUST_TARGETS = ["x86_64-unknown-linux-gnu", "x86_64-unknown-linux-musl", "x86_64-pc-windows-gnu"]
USER = getpass.getuser()

def build_dev():
    if os.path.exists("build"):
        shutil.rmtree("build")
    os.makedirs("build")
    os.makedirs("build/release")
    for target in RUST_TARGETS:
        try:
            os.makedirs("build/{}".format(target))
            sp.run(["cargo", "build", "--release", "--target", target])
            shutil.move("target/{}/release/mateus-image".format(target), "build/{}/mateus-image".format(target))
            sp.run(["tar", "-czf", "build/release/{}.tar.gz".format(target), "build/{}".format(target)])
        except:
            print("Couldn't build for target {}".format(target))

def build_local():
    if os.path.exists("build"):
        shutil.rmtree("build")
    os.makedirs("build")
    os.makedirs("build/release")
    sp.run(["cargo", "build", "--release"])
    print("Do you want to put mateus-image binary in ~/.local/bin? (works only on Linux/XDG compatible path system)")
    c = input("[y/n]: ")
    if c == "y":
        if not os.path.exists("/home/{}/.local/bin".format(USER)):
            print("~/.local/bin doesn't exists")
        else:
            shutil.move("target/release/mateus-image", "/home/{}/.local/bin/mateus-image".format(USER))
    print("done")

def main():
    print("Welcome to mateus-image's building script")
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
