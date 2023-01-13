# Types and Values

As a strongly-typed scripting language, every value in Yonscript is of a certain data type. There are two data type subsets: scalar and compound.

## Scalar types

| Type                      | Keyword       | 
|---------------------------|---------------|
| 32-bit Integer            | `number`      |
| 64-bit Integer            | `i64`         |
| Unsigned 32-bit Integer   | `u32`         |
| Unsigned 64-bit Integer   | `u64`         |
| 32-bit Floating Point     | `fractional`  |
| 64-bit Floating Point     | `f64`         |
| Boolean                   | `bool`        |
| Character                 | `char`        |
| Duration                  | `duration`    |

### Duration

In Yonscript programming language, duration is a primitive type. Internally it is stored as a unsigned 64 bit integer representing a time duration in nanoseconds, therefore it could store a time duration up to 580 years and convertable to `u64` using type casts.

`duration` values can be hard-coded with (Golang's duration string)[https://pkg.go.dev/time#ParseDuration], such as `10d`, `1.5s`, `20.32ms`, `1h20s`, etc.

### Inference

The interpreter could infer hard-coded values on a Yonscript document, so explicit type-casting are not required.

Types not covered in the table below requires an explicit type-cast.

| Type                      | Description                                                   | Examples (comma separated)    | 
|---------------------------|---------------------------------------------------------------|-------------------------------|
| 32-bit Integer            | Any natural number within `i32` boundary                      | 12, 0, -34, 343256            |
| 64-bit Integer            | Any natural number within `i64` boundary appended with `l`    | 12l, 0l, -34l, 343256l        |
| Unsigned 32-bit Integer   | Any natural number within `u32` boundary appended with `u`    | 12u, 0u, 34345u               |
| Unsigned 64-bit Integer   | Any natural number within `u64` boundary appended with `ul`   | 12ul, 0ul, 34345ul            |
| 32-bit Floating Point     | Any number within `f32` boundary appended with `f`            | 0f, 12f, 23.35f -346.34f      |
| 64-bit Floating Point     | Any number within `f64` boundary appended with `d`            | 0d, 12d, 23.35d -346.34d      |
| Boolean                   | Lowercase `true` or `false` without quotes                    | `true`, `false`               |
| Character                 | Any character with single quotes                              | `'a'`, `'3'`                  |

Compound types can also be inferred by the interpreter, in the examples below `T` will be used as the element type used within the compound type.

## Compound types 

Compound types are a group of multiple other types into one type. 

Compound types are divided into *value compound types* and *reference types*.

| Type                    | Value/Reference | Type Definition           |
|-------------------------|-----------------|---------------------------|
| Tuple                   | Value           | `(T1, T2, ..., Tn)`       |
| Matrix                  | Value           | `(T, m, n)`               |
| Struct                  | Value           | User-defined              |
| String                  | Reference       | `string`                  |
| Array                   | Reference       | `[T]`                     |
| Set                     | Reference       | `{T}`                     |
| Map                     | Reference       | `[T1,T2]`                 |
| Object                  | Reference       | User-defined              |

### Tuples 

A tuple is a list of values with a fixed size and types. A tuple data type is defined with parantheses `(T1, T2, ... Tn)` where T is another type.

| Example                               | Tuple Type                        | 
|---------------------------------------|-----------------------------------|
| `("Jane", 23)`                        | `(string, i32)`                   |
| `(2f, -2u, true, ['a', 'b', 'c'])`    | `(f32, u32, bool, []char)`        |

In Yonscript, tuples are value types.

### Matrixes 

`(` and `)` are used to define matrix values. Empty spaces are used to separate elements within a row, and `;` is used to separate rows.

```
a: (1 2 3; 4 5 6)
```

Represents a 2x3 matrix of 32-bit integers: 

```
1 2 3
4 5 6
```

---

```
b: (1f 2f 3f)
```

Represents a 1x3 matrix of 32-bit floats.

---

It's also possible to declare a one-column matrix with spaces by appending `v` in front of the matrix value declaration: 

```
# Represents a 3x1 matrix of 32-bit floats
c: v(1f 2f 3f)
```

### Structs

A struct is a composite data type of a grouped list of variables under one data type. Structs are value-types, and therefore they act like other value types such as integers and floats / fractionals. They're copied when passed along to a function or assignment. However there are some limitations with structs:
- 

```
define struct Item: 
    name: string
    value: fractional

define struct Player:
    name: string
    hp: fractional
    armor: fractional
    level: int
    items: [Item]
```

Yonscript does not support methods, as an alternative, assign the struct as a function argument instead to imitate method behaviour. There is no access specifiers either, all fields are public.

### Arrays 

An array is a list of values of the **same type** stored in a sequential order with dynamic capacity. The type is defined as `[type]`.

| Example                   | Inferred type | 
|---------------------------|---------------|
| `[1, 2, 3]`               | `[i32]`       |
| `[2f, 5.3f, -22.45f, 0f]` | `[f32]`       |
| `["hello", "world!"]`     | `[string]`    |

An array is a reference type in Yonscript, e.g. its value is not copied when it is assigned to a variable or passed around in functions. Therefore it is suitable for a long-lived variables or collection with larger cardinality. For short-lived collection of items it is recomended to use tuples instead as it is a value type.

#### Accessing Arrays

Items of an array can be access with the `[]` by its index. The index is zero-based (the first element is accessed by `array[0]`).

```
arr = [1, 2, 3]
print arr[1]    # will print 2
```

#### Appending Arrays

`append<T>(array []T, item T)` will append a new item into the array. The length of the array will be increased by 1.

```
arr = [1, 2, 3]
arr = append(arr, 4)
print arr   # will print [1, 2, 3, 4]
```

#### Merging Arrays 

`merge<T>(array [T], other [T])` will append the given array into the original array. 

```
arr = [1, 2, 3]
arr = merge(arr, [4, 5])
print arr   # will print [1, 2, 3, 4, 5]
```

### Sets 

A set is a collection of values of the same type that is unordered and does not allow duplicate values. A set data type is defined with curly braces `{T}` where T is the element type. 

| Example                               | Set Type                        | 
|---------------------------------------|-----------------------------------|
| `{string}`                            | `{"hello", "this", "name"}`       |

### Maps 

Maps are compound types that consists of key and values. A map could be declared using a list of keys and values separated by `:`

```
mapValue:
    intValue: 23
    stringValue: "heyy"
    arrayValue: [1, 2, 3]
```

```
function init():
    engine.HookEvent (
        event: engine.Event.Damage
        hook: engine.Hooks.DamageMultiplier(2)
        listener: FireWeaknessDamageEffect
        filter:
            type: engine.Damage.Fire
            target:
                element: 
                    - engine.Element.Nature
    )

routine FireWeaknessDamageEffect(damage engine.EventDamage): 
    # Flash the screen 
    engine.Screen.Flash (
        duration: 0.5s
        color: FF0000
        tween: engine.Tween.Out
    )

    # Shake the screen
    engine.Screen.Shake (
        duration: 0.3s
    )

    parallel {

    }
    # Spawn fire on top of the entity
    engine.Entity.Spawn (
        position: damage.Target.Position
        class: engine.Effect.Fire
    ) 
end
```

#### Optional Fields

Fields can be set as optional with the `?:` separator instead of the `:`

```
define struct Person
    name: string
    gender: string optional default="Unspecified"
```



### Functions

A function is a piece of code that can be executed. It is treated as a value, hence it can be stored in variables and passed around as an argument for another function.

Function is declared with the `function` keyword.

```
# A function that returns the square product of the given number
square: function (x: number) -> number 
    return x * x

# A function that applies the given function to all elements of the given array of numbers
mapNumbers: function (arr: [number], f: (number) -> number) 
    foreach item in arr 
        f(item)

# A function that receives nothing, and returns nothing, but prints out "executed!" when executed
exec: function ()
    print "executed!"

print square(10)    # prints 100
```

## Custom Types

Custom types can be defined using the `def` keyword, for both scalar and compound types.

### Custom Scalars

```
# Define a custom number type that only allows positive value
def number positive:
    validate: function (value) :: value >= 0 
    fallback: 0

# Define a custom number type that only allows positive value that is less than 100
def number positive:
    validate: function (value) :: value >= 0 && value < 100
    fallback: function (value) 
        if value < 0 return 0
        return 100

```

#### Asserting type to a struct

## Example 

```
def number Port 
```