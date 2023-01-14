#!/usr/bin/env python3

import argparse
import pathlib
import subprocess
import sys
from typing import List, Any, Optional, TextIO, Tuple

def run(*args: str, msg: Optional[str] = None, verbose: bool = False, **kwargs: Any) -> subprocess.Popen:
    sys.stdout.flush()
    if verbose:
        print(f"$ {' '.join(args)}")

    p = subprocess.Popen(args, **kwargs)
    code = p.wait()
    if code != 0:
        err = f"\nfailed to run: {args}\nexit with code: {code}\n"
        if msg:
            err += f"error message: {msg}\n"
        err(err)
    return p

def run_pipe(*args: str, msg: Optional[str] = None, verbose: bool = False, **kwargs: Any) -> TextIO:
    p = run(*args, msg=msg, verbose=verbose, stdout=subprocess.PIPE, universal_newlines=True, **kwargs)
    return p.stdout

def find_command(command: str, msg: Optional[str] = None) -> str:
    return run_pipe("which", command, msg=msg).read().strip()

def err(msg: str): 
    print(f"ERROR: {msg}")
    exit(1)

def build(debug: bool, release: bool, **kwargs: Any) -> None:
    if debug == False and release == False:
        err("please specify --debug or --release")
    
    cmake = find_command("cmake", msg="CMake is required")
    cmake_options = ["-H.", "-Bbuild"]

    if debug:
        build_cmake(cmake, "debug", "-DCMAKE_BUILD_TYPE=Debug", **kwargs)
    if release:
        build_cmake(cmake, "release", "-DCMAKE_BUILD_TYPE=RelWithDebInfo", **kwargs)

def build_cmake(cmake: str, buildpath: str, args: str, **kwargs: Any) -> None:
    basedir = pathlib.Path(__file__).parent.absolute()
    buildpath = f"build/{buildpath}"
    cmake_options = ["-H.", f"-B{buildpath}", args]
    run(cmake, str(basedir), *cmake_options, **kwargs)

    cmake_options = ["--build", buildpath]
    run(cmake, *cmake_options, **kwargs)

def parse_global_args(parser: argparse.ArgumentParser) -> None:
    parser.add_argument("-v", "--verbose", default=False, action='store_true')

if __name__ == '__main__':
    parser = argparse.ArgumentParser()


    subparsers = parser.add_subparsers()

    # build sub-menu
    parser_build = subparsers.add_parser(
        'build',
        description="Build executables",
        help="Build executables",
        formatter_class=argparse.ArgumentDefaultsHelpFormatter,
    )
    parser_build.set_defaults(func=build)
    parser_build.add_argument('-d', '--debug', default=False, action='store_true')
    parser_build.add_argument('-r', '--release', default=False, action='store_true')
    parse_global_args(parser_build)
    args = parser.parse_args()
    arg_dict = dict(vars(args))
    del arg_dict['func']
    args.func(**arg_dict)