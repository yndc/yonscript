# Routines

Routines are like functions but they are **asynchronous**. Which means that they could pause their execution mid-way while waiting for something else. A routine can be defined like a normal function but with `routine` keyword

```
DelayedHello: routine () => {
    print "Hello.."
    wait 1s
    print "..World!"
}

DelayedHello()
```

In the example above, we're waiting for 1 second before continuing the program to print the `"..World!"`. 

`wait` is a special keyword to wait for a routine to finish before continuing execution, it could receive a `duration` data type to wait for the given duration or another routine to finish.

We can see how the `wait` keyword can be used to wait for another routine to finish in the example below:

```
routine InnerWait {
    print "Inner wait start"
    wait 1s
    print "Inner wait finish"
}

routine OuterWait {
    print "Outer wait start"
    wait InnerWait()
    wait 1s
    print "Outer wait finish"
}
```

## Lifetime

When a routine is killed, all child routines will be killed too.