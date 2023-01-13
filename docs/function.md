# Functions

Functions are values in Yonscript, therefore they could be stored in variables. Functions are defined using the lambda notation `(arguments...) => { ... }`:

```
quadratic: (x: number) => x * x 
```

Functions are treated as a value-type since they are immutable in compile-time.

User-defined structures such as `struct` and `object` can have functions as their member, with the type signature: 

```
(T1, T2, ..., Tn) => TR
# Where T1..TN is the type of the argument and TR is the type of the returned value
```