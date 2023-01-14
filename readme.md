# Yonscript

Yonscript is a simple strongly-typed markup and scripting language designed for defining game assets and logic.

Yonscript code is compiled to byte-code by the Yonscript compiler, which is executable by the Yonscript virtual machine written in **C++**.

This is a mono-repository for all Yonscript projects, including the bytecode compiler, virtual machine, CLI tool, and documentation.

## Building 

### Linux 

1. Install dependencies

Debian/Ubuntu
```
sudo apt install libfmt-dev
```

2. Build 

```
./build.sh
```

or 
``