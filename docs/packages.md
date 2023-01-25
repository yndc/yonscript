# Modules

Outside of the REPL, all code must be be located inside a package folder. A package folder is any folder with a package metadata file, with the name `yonscript.package.yaml`. The compiler can't compile any source file outside a package folder. 

## In-module Scoping 

Declarations of functions, events, structures, states, etc are required to be exposed with the `expose` keyword to make them visible to other files. 

```
// first/one.ys

expose function A () => 
    print "A is called!"
```

Source files within the same package can be imported through relative paths and absolute paths. 

Relative paths uses the importer file location as the origin.  

```
// first/two.ys

import ./one.ys as One

expose function B () => 
    print "B is called!"

// main.ys 
use IO 
import ./first/one.ys   as One
import ./first/two.ys   as Two

handle event Start
    () => Empty 
        One.A()
        Two.B()
        print("Main is called!")
```

Absolute paths uses the `yonscript.package.yaml` as the path origin. 

The module alias is required. Exported functions and events can be referenced using the `.` operator. 

## External modules 

External modules can be included within a package in `yonscript.package.yaml` under `dependencies`. It's a map of module name with its git commit. The CLI tool will clone all packages in the dependencies section using git. 

```
dependencies:
    <module-name>: <git url>
    ...
```

The module name will be used for importing the code inside the current package.

```
// yonscript.package.yaml
dependencies:
    SomePackage: <git url>

// main.ys
use SomePackage

handle event Start
    () => None 
        SomePackage.SomeFunction()
```