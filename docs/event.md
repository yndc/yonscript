# Events 

Yonscript has a built-in event system for writing event handlers flexibly and efficiently. 

## Definition 

Events can be defined with `define event` keyword:

```
define event <event name> {
    <key>: <type>
    ...
}
```

### Attributes 

#### (optional) `delivery`
 
Type: `enum: ExactlyOnce | AtMostOnce`

Default: `ExactlyOnce`

Enforce the delivery guarantee of the event. 

`ExactlyOnce` guarantees that the event will be delivered* to all of the recipients exactly once. This is the most expensive option as the event packet will have to be supplied with event identifiers and the engine will need to check for duplicates. Acknowledgements from the recipients is also required. Recomended for important events where a exactly-once semantics is necessary.

`AtMostOnce` guarantees that the event will be delivered to all of the recipients at most once. The publisher will publish the event once, if for some reason an event fails to be recieved by a consumer, the event won't be processed by that consumer. This is the fastest option as there is no additional event metadata associated in the event packet and no acknowledgement mechanism is necessary. 

*The event packet itself does not have an exactly-once guarantee. The engine will deduplicate the events before forwarding them to the engine.

#### (optional) `order`

Type: `EventOrder`

Enforce ordering of the events. Event data will be supplied with a sequential order number, the engine will buffer the received event `n` until it has processed the `(n-1)` event. When using an order guarantee, the `delivery` attribute must be `ExactlyOnce`.

`EventOrder` is a special reference type that is used for differentiating event orders. The engine will enforce ordering of all events with the same `EventOrder` reference. Events with the same `EventOrder` reference will be guaranteed to be processed in order as they were emitted. 

There is two built-in `EventOrder`:

- `Global` enforces a global ordering of the event regardless of the type. All events under the `Global` order will be processed sequentially.

- `Type` enforces ordering of the same event type.

Event ordering might be essential in some scenarios, especially when an event depends on another event before it. However event ordering introduces a hefty performance penalty, as events in the same `EventOrder` cannot be processed in a concurrent manner. Therefore it is wise to use event ordering sparingly, only on monumental events where ordering matters.

## Hooks

An event handler is a function that is executed when an event is being emitted. Event handlers cannot change the event data itself, and by default they are run in parallel. So if there are 20 handlers for a specific event, 20 of them will be run in parallel when the specific event is emitted. 

Event hooks on the other hand, are run sequentially. Also, they could change the event data itself.  

```
# Define the event data
define event IncomingMessage {
    message: string
    sender: string
}

# Write a hook that appends the sender into the message (event mutation happens)
senderAppender: hook IncomingMessage (event) => {
    event.message = `{event.sender} says {event.message}`
    return event
}

# Write a handler that prints out the message to console
messagePrinter: handle IncomingMessage (event) => {
    IO.PrintString(event.message)
}

# Emit events using the `emit` keyword
emit IncomingMessage{ message: "hello!", sender: "yonder" }     # Will print "yonder says hello!"
emit IncomingMessage{ message: "world!", sender: "yonder" }     # Will print "yonder says world!"
emit IncomingMessage{ message: "hi!", sender: "socks" }         # Will print "socks says hi!"
emit IncomingMessage{ message: "bye!", sender: "evan" }         # Will print "evan says bye!"

# Stop the event handlers
stop senderAppender
stop messagePrinter

# Emit more events, but since the handlers have gone, nothing will happen
emit IncomingMessage{ message: "I'm alone..", sender: "loner" }

```

### Blocking events

An event hook could also block the event altogether by using the keyword `block` instead of `return`

```
define event IncomingMessage {
    message: string
    sender: string
}

messagePrinter: handle IncomingMessage (event) => {
    IO.PrintString(event.message)
}

messageBlocker: hook IncomingMessage (event) => {
    # Block the event if the sender is "loner"
    if event.sender == "loner" {
        block
    }
}

emit IncomingMessage{ message: "hello..?", sender: "loner" }        # Will not be printed
emit IncomingMessage{ message: "sup guys", sender: "coolguy" }      # Will be printed
```

### Priority

By default multiple hooks are ordered based on the order they were registered.

```
define event SomeEvent

aHook: hook SomeEvent (event) => IO.Print("a!")
bHook: hook SomeEvent (event) => IO.Print("b!")

emit SomeEvent      # Will print "a!" then "b!" because `aHook` is registered first
```

This behaviour can be overriden by supplying a priority number on the `hook` keyword. The priority number is a 32-bit integer, which supports negative values. Hooks without a priority number has an effective 0 priority number. Hooks with the same priority number will be ordered based on which one registered first.

```
define event SomeEvent

a: hook SomeEvent (event) => IO.Print("a!")
b: hook SomeEvent (event) => IO.Print("b!")
c: hook SomeEvent (event) with priority -100 => IO.Print("c!")
d: hook SomeEvent (event) with priority 100 => IO.Print("d!")
e: hook SomeEvent (event) with priority 100 => IO.Print("e!")
f: hook SomeEvent (event) with priority 0 => IO.Print("f!")

emit SomeEvent      # Will print "d!", "e!", "a!", "b!", "f!", and "c!" in that order
```

### When to use hooks?

Use hooks only if you need to alter the event data or block the event altogether. Hooks are run in sequence, the next hook couldn't be run before the current one finishes. Therefore they can't be optimized using multiple threads. Normal handlers however are run in parallel and therefore more efficient.  

## Filters

Event handlers can be supplied with a filter to make it handles only events that passes the condition. 

```
define event Temperature {
    value: number
}

tempAlert: handle Temperature 
    filter value > 50
    (event) => IO.Print("High temperature detected: {event.value}")

emit Temperature { value: 23 }      # Prints nothing
emit Temperature { value: 45 }      # Prints nothing 
emit Temperature { value: 65 }      # Prints "High temperature detected: 65" 
```

What's the difference with putting an `if` condition inside the event handler itself? The event engine will index the filters and group handlers with the same filters together for efficiency.