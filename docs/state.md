# State 

Yonscript has a restriction on global variables: they are immutable. 


```
# A state can be created through a structure. 
state Counter 
    counter             Integer
    lastIncremented     Time

# A state can be created with the `new` keyword. 
new Counter

# A system could process and modify states 
system CounterIncrementer 
    mutates     state   Counter
    handles     event   Increment
    (entity, event) => 
        entity[Counter].counter += 1
```


```
```