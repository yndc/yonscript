# Lifetimes 

There are two kinds of types in Yonscript, value-types and reference types. This is similar in other programming languages with the same terminology, i.e. value types are stored in the stack while reference types are stored in the heap. 

Yonscript does not have a garbage collector, therefore heap memory management is done through variable lifetimes like those in Rust although more limited and hence simpler.

In short, memory allocation in done on data initialization and it is freed when the variable is out of scope.

```
ReadAndPrintConsole: function () { 
    # Read string from the input buffer (typically console)
    a = IO.ReadString()     # memory is allocated here, variable a lifetime begins here

    IO.PrintString(a)       # variable a is borrowed by the IO.PrintString function

    # a is destroyed here
}
```

The default behaviour of passing a reference into function arguments is the reference is being borrowed by the function. Therefore by default, reference-types in function arguments are read-only. 