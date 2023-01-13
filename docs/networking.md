# Networking

Yonscript has several built-in features for multi-instance scenarios, such as in multiplayer games. The host engine is responsible for connecting multiple Yonscript instances together. 

This page is a high-level overview of the built-in networking features written for Yonscript users. Read more in the [Networking Integration]() section of the documentation for the technical details around networking.

## Events 

Events are the main method of communication between Yonscript instances. By default, events emitted from one instance will be broadcasted to all instances, and they adhere to exactly-once 