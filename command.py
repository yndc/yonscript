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
        raise RuntimeError(err)

    return p

def run_pipe(*args: str, msg: Optional[str] = None, verbose: bool = False, **kwargs: Any) -> TextIO:
    p = run(*args, msg=msg, verbose=verbose, stdout=subprocess.PIPE, universal_newlines=True, **kwargs)
    return p.stdout  # type: ignore

def find_command(command: str, msg: Optional[str] = None) -> str:
    return run_pipe("which", command, msg=msg).read().strip()

def build() -> None:
    basedir = pathlib.Path(__file__).parent.absolute()
    cmake = find_command("cmake", msg="CMake is required")
    print(basedir)

if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    subparsers = parser.add_subparsers()
    parser_build = subparsers.add_parser(
        'build',
        description="Build executables",
        help="Build executables",
        formatter_class=argparse.ArgumentDefaultsHelpFormatter,
    )
    parser_build.set_defaults(func=build)
    args = parser.parse_args()
    arg_dict = dict(vars(args))
    del arg_dict['func']
    args.func(**arg_dict)