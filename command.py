#!/usr/bin/env python3

import argparse
import pathlib
import subprocess
import sys
from typing import List, Any, Optional, TextIO, Tuple

def run(*args: str, msg: Optional[str] = None, verbose: bool = False, **kwargs: Any) -> subprocess.Popen:
    sys.stdout.flush()
    # if verbose:
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

def build(debug: bool, release: bool) -> None:
    if debug == True and release == True:
        err("both --debug and --release cannot be used at the same time")
    if debug == False and release == False:
        err("please specify --debug or --release")

    basedir = pathlib.Path(__file__).parent.absolute()
    cmake = find_command("cmake", msg="CMake is required")

    run(cmake, str(basedir), cwd=dir)

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
    args = parser.parse_args()
    arg_dict = dict(vars(args))
    del arg_dict['func']
    print(arg_dict)
    args.func(**arg_dict)